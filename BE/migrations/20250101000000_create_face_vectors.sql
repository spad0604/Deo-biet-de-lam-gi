-- =========================================
-- Bảng Face Vectors để lưu embedding vectors
-- =========================================
-- Lưu ý: Bảng này tham chiếu đến bảng students (nếu có) hoặc student
-- Cần kiểm tra tên bảng chính xác trong database của bạn
CREATE TABLE IF NOT EXISTS face_vectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL,  -- Sẽ thêm FOREIGN KEY sau khi xác nhận tên bảng
    embedding FLOAT[] NOT NULL,  -- Array để lưu vector [0.123, 0.456, ...]
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE(student_id)  -- Mỗi học sinh chỉ có 1 vector (có thể bỏ nếu muốn nhiều vector)
);

-- Index để tìm kiếm nhanh
CREATE INDEX IF NOT EXISTS idx_face_vectors_student_id ON face_vectors(student_id);

-- Nếu bảng là 'students' (plural):
-- ALTER TABLE face_vectors ADD CONSTRAINT fk_face_vectors_student_id 
-- FOREIGN KEY (student_id) REFERENCES students(id) ON DELETE CASCADE;

-- Nếu bảng là 'student' (singular):
-- ALTER TABLE face_vectors ADD CONSTRAINT fk_face_vectors_student_id 
-- FOREIGN KEY (student_id) REFERENCES student(id) ON DELETE CASCADE;

