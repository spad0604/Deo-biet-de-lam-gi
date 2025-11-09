import numpy as np
import datetime
import threading

db_lock = threading.Lock()

known_face_db = {
    "ids": [],
    "vectors": np.empty((0, 128))
}

students_checked_in_today = set()
last_reset_day = datetime.date.today().day

def reset_checkin_list_if_new_day():
    """
        Tự động xóa danh sách điểm danh khi qua ngày mới.
    """
    global students_checked_in_today, last_reset_day
    today = datetime.date.today().day

    if today != last_reset_day:
        with db_lock:
            if datetime.date.today().day != last_reset_day:
                print(f"Qua ngày mới. Xóa danh sách điểm danh...")
                students_checked_in_today.clear()
                last_reset_day = datetime.date.today().day