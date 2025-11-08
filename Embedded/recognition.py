import cv2
import numpy as np
from deepface import DeepFace
from config import MODEL_NAME, DETECTOR_BACKEND, FACE_RECOGNITION_THRESHOLD
from state import known_face_db, db_lock

def get_embedding_from_image(image_bytes: bytes) -> list:
    """
        Nhận bytes của ảnh, trả về vector embedding.
    """
    try:
        nparr = np.frombuffer(image_bytes, np.uint8)
        img = cv2.imdecode(nparr, cv2.IMREAD_COLOR)

        if img is None:
            raise ValueError("Invalid image.")

        embedding_obj = DeepFace.represent(
            img_path=img,
            model_name=MODEL_NAME,
            enforce_detection=True,
            detector_backend=DETECTOR_BACKEND
        )
        return embedding_obj[0]["embedding"]

    except ValueError as e:
        raise e
    except Exception as e:
        raise e

def find_best_match(embedding_vector: np.ndarray) -> (str | None, float):
    """
        So sánh vector đầu vào với toàn bộ CSDL trong RAM.
        Trả về (student_id, distance).
    """
    with db_lock:
        if known_face_db["vectors"].shape[0] == 0:
            return None, float("inf") #CSDL rong

        # (1) Lấy mảng CSDL
        db_vectors = known_face_db["vectors"]
        db_ids = db_lock["ids"]

    # (2) Tính toán khoảng cách (Euclidean L2)
    diff = db_vectors - embedding_vector
    distance = np.linalg.norm(diff, axis=1)

    # (3) Tìm khoảng cách nhỏ nhất
    min_distance_index = np.argmin(distance)
    min_distance = distance[min_distance_index]

    # (4) Kiểm tra ngưỡng
    if min_distance < FACE_RECOGNITION_THRESHOLD:
        student_id = db_ids[min_distance_index]
        return student_id, min_distance
    else:
        return None, min_distance  # Không khớp ai

