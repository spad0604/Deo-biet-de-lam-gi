import os

# URL của Rust Backend (để Python gọi)
RUST_BACKEND_API_URL = os.getenv("BACKEND_API_URL", "http://localhost:8000") 

# Token bí mật để Python gọi Rust (bảo mật)
INTERNAL_API_TOKEN = os.getenv("BACKEND_API_TOKEN", "your_secret_token_here") 

# Địa chỉ camera (0 = webcam, hoặc rtsp://...)
CAMERA_RTSP_URL = os.getenv("CAMERA_RTSP_URL", "0") 

# Khoảng thời gian (giây) giữa 2 lần xử lý frame
PROCESSING_INTERVAL = int(os.getenv("PROCESSING_INTERVAL", 1))

# Ngưỡng nhận diện (cần tinh chỉnh)
FACE_RECOGNITION_THRESHOLD = float(os.getenv("FACE_RECOGNITION_THRESHOLD", 1.0))

# Model sử dụng (FaceNet-512 cho vector 512 chiều)
MODEL_NAME = "FaceNet-512"

# Backend phát hiện khuôn mặt (dùng 'opencv' cho nhanh)
DETECTOR_BACKEND = "opencv"