# app/api.py
import threading
import requests
import numpy as np
from fastapi import FastAPI, UploadFile, File, HTTPException

from config import RUST_BACKEND_API_URL, INTERNAL_API_TOKEN
from state import known_face_db, db_lock
from recognition import get_embedding_from_image
from camera_worker import process_camera_stream

# Khởi tạo ứng dụng FastAPI
app = FastAPI(
    title="Face Recognition AI Service",
    description="Service AI xử lý nhận diện khuôn mặt."
)


# ===================================================================
# HÀM KHỞI ĐỘNG (STARTUP)
# ===================================================================

def sync_database_on_startup():
    """
    Gọi API của Rust để lấy CSDL vector.
    """
    global known_face_db
    print("Đang đồng bộ CSDL vector từ Rust Backend...")
    try:
        response = requests.get(
            f"{RUST_BACKEND_API_URL}/api/internal/all-face-vectors",
            headers={"Authorization": f"Bearer {INTERNAL_API_TOKEN}"}
        )
        response.raise_for_status()
        response_data = response.json()
        
        data = response_data.get('data', []) if isinstance(response_data, dict) else response_data

        if not data:
            print("CSDL vector rỗng.")
            new_db = {"ids": [], "vectors": np.empty((0, 128))}
        else:
            ids = [item['id'] for item in data]
            vectors = [item['vector'] for item in data]
            new_db = {
                "ids": ids,
                "vectors": np.array(vectors).astype('float32')
            }

        with db_lock:
            known_face_db = new_db

        print(f"Đồng bộ thành công. Đã tải {len(known_face_db['ids'])} vector.")

    except Exception as e:
        print(f"LỖI ĐỒNG BỘ CSDL: {e}. Chạy với CSDL rỗng.")
        with db_lock:
            known_face_db = {"ids": [], "vectors": np.empty((0, 128))}


@app.on_event("startup")
def on_startup():
    # 1. Chạy đồng bộ CSDL
    threading.Thread(target=sync_database_on_startup, daemon=True).start()

    # 2. Chạy vòng lặp camera
    print("Khởi chạy Camera Processing Thread...")
    camera_thread = threading.Thread(target=process_camera_stream, daemon=True)
    camera_thread.start()


# ===================================================================
# API ENDPOINTS
# ===================================================================

@app.get("/")
def read_root():
    return {"status": "AI Service is running"}


@app.post("/api/ai/extract-embedding")
async def extract_embedding(file: UploadFile = File(...)):
    """
    API này để Rust gọi khi đăng ký học sinh mới.
    Nhận ảnh -> Trả về vector.
    """
    try:
        contents = await file.read()
        embedding_vector = get_embedding_from_image(contents)
        return {"status": "success", "embedding": embedding_vector}

    except ValueError as e:  # Lỗi không tìm thấy mặt
        raise HTTPException(status_code=400, detail=f"Không tìm thấy khuôn mặt: {str(e)}")
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Lỗi máy chủ nội bộ: {str(e)}")