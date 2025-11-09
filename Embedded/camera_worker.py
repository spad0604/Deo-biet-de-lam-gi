import cv2
import time
import datetime
import requests
import numpy as np
from deepface import DeepFace
from config import CAMERA_RTSP_URL, PROCESSING_INTERVAL,RUST_BACKEND_API_URL, INTERNAL_API_TOKEN, MODEL_NAME, DETECTOR_BACKEND

from state import students_checked_in_today, db_lock, reset_checkin_list_if_new_day
from recognition import find_best_match

def process_camera_stream():
    """
        Hàm này chạy trong thread riêng.
        Kết nối camera, xử lý frame, và gọi API của Rust khi cần.
    """

    print("Processing camera stream...")
    cap = cv2.VideoCapture(0 if CAMERA_RTSP_URL == "0" else CAMERA_RTSP_URL)

    if not cap.isOpened():
        print("Camera stream not opened")
        return

    while True:
        try:
            reset_checkin_list_if_new_day()

            ret, frame = cap.read()
            if not ret:
                print("Camera stream not readable")
                time.sleep(5)
                cap.release()
                cap = cv2.VideoCapture(0 if CAMERA_RTSP_URL == "0" else CAMERA_RTSP_URL)
                continue

            try:
                all_faces = DeepFace.extract_faces(
                    img_path=frame,
                    detector_backend=DETECTOR_BACKEND,
                    enforce_detection=False
                )

            except Exception:
                continue

            for face_obj in all_faces:
                if face_obj['confidence'] < 0.9:
                    continue
                face_img = face_obj['face']
                try:
                    face_img_normalized = face_img / 255.0
                    embedding_obj = DeepFace.represent(
                        img_path=face_img_normalized,
                        model_name=MODEL_NAME,
                        enforce_detection=False
                    )
                    embedding_vector = np.array(embedding_obj["embedding"])

                except Exception:
                    continue

                student_id, distance = find_best_match(embedding_vector)

                if student_id:
                    with db_lock:
                        is_already_checked_in = student_id in students_checked_in_today

                    if not is_already_checked_in:
                        # 6. GỌI API RUST
                        print(f"PHÁT HIỆN: {student_id} (Dist: {distance:.2f}). Gửi check-in...")
                        send_checkin_to_rust(student_id)

            time.sleep(PROCESSING_INTERVAL)

        except Exception as e:
            print(e)

    cap.realse()

def send_checkin_to_rust(student_id : str):
    """
        Gửi thông báo check-in đến Rust và thêm vào set
    """
    try:
        timestamp = datetime.datetime.now().isoformat()
        response = requests.post(
            f"{RUST_BACKEND_API_URL}/api/internal/check-in",
            json={"student_id": student_id, "timestamp": timestamp},
            headers={"Authorization": f"Bearer {INTERNAL_API_TOKEN}"},
            timeout=2
        )
        response.raise_for_status()

        with db_lock:
            students_checked_in_today.append(student_id)
        print(f"Đã gửi check-in thành công cho {student_id}.")

    except Exception as e:
        print(f"LỖI khi gọi API check-in của Rust cho {student_id}: {e}")