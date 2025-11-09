-- =========================================
-- 1️⃣ Bảng ClassRoom
-- =========================================
CREATE TABLE class_room (
                            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                            name TEXT NOT NULL
);

-- =========================================
-- 2️⃣ Bảng Teacher
-- =========================================
CREATE TABLE teacher (
                         id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                         first_name TEXT NOT NULL,
                         last_name TEXT NOT NULL,
                         date_of_birth TIMESTAMPTZ NOT NULL,
                         phone_number TEXT NOT NULL,
                         image_url TEXT NOT NULL,
                         role TEXT NOT NULL DEFAULT 'Teacher', -- giữ Role dạng text
                         homeroom_class_id UUID REFERENCES class_room(id) ON DELETE SET NULL
);

-- =========================================
-- 3️⃣ Bảng Student
-- =========================================
CREATE TABLE student (
                         id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                         first_name TEXT NOT NULL,
                         last_name TEXT NOT NULL,
                         date_of_birth TIMESTAMPTZ NOT NULL,
                         phone_number TEXT NOT NULL,
                         image_url TEXT NOT NULL,
                         role TEXT NOT NULL DEFAULT 'Student',
                         class_id UUID NOT NULL REFERENCES class_room(id) ON DELETE CASCADE
);

-- =========================================
-- 4️⃣ (Tuỳ chọn) Bảng TeacherSubject
-- =========================================
CREATE TABLE teacher_subject (
                                 teacher_id UUID NOT NULL REFERENCES teacher(id) ON DELETE CASCADE,
                                 subject_id UUID NOT NULL,
                                 PRIMARY KEY (teacher_id, subject_id)
);

CREATE TABLE subject (
                         id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                         name TEXT NOT NULL
);

INSERT INTO class_room (name) VALUES
                                  ('10T1'), ('10T2'), ('10Tin'), ('10S'), ('10H'), ('10L'), ('10V'), ('10A1'), ('10A2'), ('10P'), ('10Su'), ('10Dia'), ('10Trung');

INSERT INTO class_room (name) VALUES
                                  ('11T1'), ('11T2'), ('11Tin'), ('11S'), ('11H'), ('11L'), ('11V'), ('11A1'), ('11A2'), ('11P'), ('11Su'), ('11Dia'), ('11Trung');

INSERT INTO class_room (name) VALUES
                                  ('12T1'), ('12T2'), ('12Tin'), ('12S'), ('12H'), ('12L'), ('12V'), ('12A1'), ('12A2'), ('12P'), ('12Su'), ('12Dia'), ('12Trung');

INSERT INTO subject (name) VALUES ('Toán học');
INSERT INTO subject (name) VALUES ('Ngữ văn');
INSERT INTO subject (name) VALUES ('Vật lý');
INSERT INTO subject (name) VALUES ('Hóa học');
INSERT INTO subject (name) VALUES ('Sinh học');
INSERT INTO subject (name) VALUES ('Lịch sử');
INSERT INTO subject (name) VALUES ('Địa lý');
INSERT INTO subject (name) VALUES ('Tin học');
INSERT INTO subject (name) VALUES ('Thể dục');
INSERT INTO subject (name) VALUES ('Tiếng Anh');
INSERT INTO subject (name) VALUES ('Tiếng Pháp');
INSERT INTO subject (name) VALUES ('Tiếng Trung');
