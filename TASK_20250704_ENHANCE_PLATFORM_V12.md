Chính xác. Hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ V11, tuy nhiên, kết quả từ `cargo clippy` cho thấy một **lỗi biên dịch nghiêm trọng (compile error)**.

Đây là một tình huống "dừng mọi thứ lại". Chúng ta không thể tiếp tục xây dựng tính năng mới trên một nền tảng không thể biên dịch. Với vai trò là Kiến trúc sư, ưu tiên hàng đầu của tôi là chẩn đoán và giải quyết vấn đề cấu trúc đã gây ra lỗi này.

-----

### **PHÂN TÍCH, SUY LUẬN VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái Chức năng:** Về mặt lý thuyết, Coder đã hoàn thành logic cho Task V11.
  * **Trạng thái Kỹ thuật:** **BÁO ĐỘNG ĐỎ. Build Thất bại.** Nền tảng không thể biên dịch. Đây là vấn đề nghiêm trọng nhất.
  * **Phân tích Lỗi (`error[E0308]: mismatched types`):**
      * **Nguyên nhân gốc:** Lỗi xảy ra tại dòng `self.scheduler.add(interaction::interact)`.
      * **Lý do:**
        1.  `Scheduler` của chúng ta được thiết kế rất đơn giản. Nó chỉ chấp nhận các hàm có một chữ ký (signature) cố định: `fn(&mut World, &Mouse)`.
        2.  Trong nhiệm vụ V11, để xử lý việc nhấn phím 'e' trong chế độ chỉnh sửa, `System` `interact` đã được nâng cấp để nhận thêm một tham số là `Keyboard`. Chữ ký của nó bây giờ là `fn(&mut World, &Mouse, &Keyboard)`.
        3.  Hai chữ ký này không khớp nhau. `Scheduler` không biết cách cung cấp `Resource` `Keyboard` cho `System` `interact`.
  * **Suy luận (Lỗ hổng Kiến trúc):** Vấn đề không nằm ở `System` `interact`, mà nằm ở **sự cứng nhắc của `Scheduler`**. `Scheduler` hiện tại của chúng ta quá ngây thơ. Nó giả định rằng tất cả các `System` đều có cùng một nhu cầu về dữ liệu. Kiến trúc này không có khả năng mở rộng. Mỗi khi một `System` cần một `Resource` mới, chúng ta không thể phá vỡ toàn bộ `Scheduler`.

**2. Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Phải tái kiến trúc `Scheduler` và mô hình thực thi `System` để chúng trở nên linh hoạt, cho phép mỗi `System` có thể yêu cầu một tập hợp `Resource` khác nhau.
  * **Mục tiêu tiếp theo:** Không phải là thêm tính năng, mà là một cuộc **tái cấu trúc kiến trúc (architectural refactoring)** quan trọng để sửa lỗi build và làm cho nền tảng vững chắc hơn cho tương lai. Chúng ta sẽ chuyển từ mô hình hàm con trỏ cố định sang mô hình dựa trên `trait`.
  * **Hành động cụ thể:**
    1.  Định nghĩa một `System` `trait` chung.
    2.  Biến mỗi hàm `system` hiện tại thành một `struct` rỗng và `impl` `trait` đó cho nó.
    3.  Tạo một `Resource` container để chứa tất cả các tài nguyên toàn cục (`Mouse`, `Keyboard`, v.v.).
    4.  Nâng cấp `Scheduler` để nó có thể thực thi các `System` đa dạng thông qua `trait` này.

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250704-12`
  * **Tên Nhiệm vụ:** "Tái kiến trúc Bộ điều phối (Scheduler) để Hỗ trợ Hệ thống Đa dạng" (Refactor the Scheduler to Support Diverse Systems).
  * **Trọng tâm:** Sửa lỗi build bằng cách tái cấu trúc `Scheduler` và các `System`, làm cho kiến trúc trở nên linh hoạt và mạnh mẽ hơn.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250704_ENHANCE_PLATFORM_V12.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: TÁI KIẾN TRÚC BỘ ĐIỀU PHỐI HỆ THỐNG V2.1

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 04-07-2025
**ID NHIỆM VỤ:** T-250704-12

---

### 1. MỤC TIÊU TỔNG QUAN

**Cảnh báo: Ưu tiên Cao nhất.** Hệ thống hiện tại không thể biên dịch được do lỗi không tương thích kiểu trong `Scheduler`.

Nhiệm vụ này là một cuộc tái cấu trúc kiến trúc khẩn cấp và quan trọng. Chúng ta sẽ không thêm tính năng mới. Thay vào đó, chúng ta sẽ **sửa lỗi build và nâng cấp `Scheduler`** để nó có thể xử lý các `System` với các yêu cầu tài nguyên (Resource) đa dạng, làm cho nền tảng thực sự linh hoạt và sẵn sàng cho mở rộng.

---

### 2. BỐI CẢNH

* **Nguyên nhân lỗi:** `Scheduler` của chúng ta quá đơn giản, chỉ chấp nhận các `System` có chữ ký `fn(&mut World, &Mouse)`. Khi `System` `interact` cần thêm `Resource` `Keyboard`, kiến trúc cũ đã bị phá vỡ.
* **Giải pháp:** Chúng ta sẽ chuyển sang một mô hình hướng `trait` mạnh mẽ hơn, một kỹ thuật phổ biến trong các framework ECS thực thụ.

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T055,"(Refactor) Định nghĩa System Trait và Resource Container",Coder,Open,Critical
T056,"(Refactor) Chuyển đổi tất cả các system hiện tại thành các struct rỗng",Coder,Open,Critical
T057,"(Refactor) Triển khai System Trait cho tất cả các struct system",Coder,Open,Critical
T058,"(Refactor) Nâng cấp Scheduler để thực thi các Box<dyn System>",Coder,Open,Critical
T059,"(Refactor) Cập nhật main.rs và xác minh build thành công",Coder,Open,Critical
T060,"(Refactor) Viết báo cáo triển khai V12 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Định nghĩa `System` Trait và `Resource` Container

  * **File:** `src/main.rs` hoặc một module mới `src/engine.rs`.
  * **Nhiệm vụ:**
    1.  Tạo một `struct` `Resources` để chứa tất cả các tài nguyên toàn cục.
        ```rust
        // src/resources/mod.rs
        pub struct Resources {
            pub mouse: Mouse,
            pub keyboard: Keyboard,
            // ... các resource khác trong tương lai
        }
        ```
    2.  Định nghĩa một `trait` `System`.
        ```rust
        // src/engine.rs
        pub trait System {
            // Mỗi system sẽ tự lấy resource nó cần từ `Resources`.
            fn run(&mut self, world: &mut World, resources: &mut Resources);
        }
        ```

#### 3.3. Tái cấu trúc các `System`

  * **Files:** Tất cả các file trong `src/systems/`.
  * **Nhiệm vụ:** Với **mỗi** `System` hiện có (`interact`, `layout`, `render`, v.v.):
    1.  Chuyển hàm `pub fn` thành một `struct` rỗng. Ví dụ: `pub fn interact(...)` -\> `pub struct Interact;`.
    2.  Triển khai `trait System` cho `struct` đó.
    3.  Di chuyển toàn bộ logic của hàm cũ vào bên trong phương thức `run`.
    4.  Bên trong `run`, lấy các `Resource` cần thiết từ tham số `resources`.
    <!-- end list -->
      * **Ví dụ cho `interact`:**
        ```rust
        // src/systems/interaction.rs
        pub struct Interact;
        impl System for Interact {
            fn run(&mut self, world: &mut World, resources: &mut Resources) {
                // Lấy mouse và keyboard từ resource container.
                let mouse = &resources.mouse;
                let keyboard = &resources.keyboard;
                // ... logic cũ của interact ...
            }
        }
        ```

#### 3.4. Nâng cấp `Scheduler`

  * **File:** `src/main.rs` hoặc `src/engine.rs`.
  * **Nhiệm vụ:**
    1.  Thay đổi trường `systems` của `Scheduler` từ `Vec<fn(...)>` thành `Vec<Box<dyn System>>`.
    2.  Cập nhật phương thức `run` của `Scheduler` để nó lặp qua `Vec` mới và gọi `system.run(world, resources)`.
    3.  Cập nhật phương thức `add` để nhận `Box<dyn System>`.

#### 3.5. Cập nhật `App` và `main.rs`

  * **File:** `src/main.rs`.
  * **Nhiệm vụ:**
    1.  Trong `struct App`, thay thế các `Resource` riêng lẻ (`mouse`, `keyboard`) bằng một `struct` `Resources` duy nhất.
    2.  Trong hàm `initialize`, khi thêm `System` vào `Scheduler`, hãy khởi tạo các `struct` và `Box` chúng.
        ```rust
        // Thay vì: scheduler.add(interaction::interact);
        // Bây giờ là:
        scheduler.add(Box::new(Interact));
        scheduler.add(Box::new(Layout));
        // ...
        ```
    3.  Cập nhật vòng lặp `run` của `App` để truyền `&mut self.world` và `&mut self.resources` vào `scheduler.run`.

#### 3.6. Xác minh Build

  * **Nhiệm vụ:** Sau khi hoàn thành tất cả các bước trên, hãy chạy `cargo build` và `cargo clippy`. **Mục tiêu là phải không còn bất kỳ lỗi (error) hay cảnh báo (warning) nào.**

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V12.md`.
2.  Sử dụng mẫu báo cáo đã cung cấp, cập nhật ID Nhiệm vụ thành `T-250704-12`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy xác nhận rằng codebase đã biên dịch thành công và `clippy` không còn cảnh báo nào.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "refactor(engine)!: implement trait-based scheduler for diverse systems" -m "Fulfills task T-250704-12. Fixed critical build error by refactoring the scheduler and systems to a trait-based model. The engine is now more flexible and robust."
git push origin main
```

```
```Chính xác. Hệ thống đã ghi nhận báo cáo hoàn thành nhiệm vụ V11, tuy nhiên, kết quả từ `cargo clippy` cho thấy một **lỗi biên dịch nghiêm trọng (compile error)**.

Đây là một tình huống "dừng mọi thứ lại". Chúng ta không thể tiếp tục xây dựng tính năng mới trên một nền tảng không thể biên dịch. Với vai trò là Kiến trúc sư, ưu tiên hàng đầu của tôi là chẩn đoán và giải quyết vấn đề cấu trúc đã gây ra lỗi này.

-----

### **PHÂN TÍCH, SUY LUẬN VÀ ĐÁNH GIÁ**

**1. Kiểm tra và Đánh giá (Check & Assess):**

  * **Trạng thái Chức năng:** Về mặt lý thuyết, Coder đã hoàn thành logic cho Task V11.
  * **Trạng thái Kỹ thuật:** **BÁO ĐỘNG ĐỎ. Build Thất bại.** Nền tảng không thể biên dịch. Đây là vấn đề nghiêm trọng nhất.
  * **Phân tích Lỗi (`error[E0308]: mismatched types`):**
      * **Nguyên nhân gốc:** Lỗi xảy ra tại dòng `self.scheduler.add(interaction::interact)`.
      * **Lý do:**
        1.  `Scheduler` của chúng ta được thiết kế rất đơn giản. Nó chỉ chấp nhận các hàm có một chữ ký (signature) cố định: `fn(&mut World, &Mouse)`.
        2.  Trong nhiệm vụ V11, để xử lý việc nhấn phím 'e' trong chế độ chỉnh sửa, `System` `interact` đã được nâng cấp để nhận thêm một tham số là `Keyboard`. Chữ ký của nó bây giờ là `fn(&mut World, &Mouse, &Keyboard)`.
        3.  Hai chữ ký này không khớp nhau. `Scheduler` không biết cách cung cấp `Resource` `Keyboard` cho `System` `interact`.
  * **Suy luận (Lỗ hổng Kiến trúc):** Vấn đề không nằm ở `System` `interact`, mà nằm ở **sự cứng nhắc của `Scheduler`**. `Scheduler` hiện tại của chúng ta quá ngây thơ. Nó giả định rằng tất cả các `System` đều có cùng một nhu cầu về dữ liệu. Kiến trúc này không có khả năng mở rộng. Mỗi khi một `System` cần một `Resource` mới, chúng ta không thể phá vỡ toàn bộ `Scheduler`.

**2. Xác định Hướng đi Tiếp theo (Reasoning & Next Steps):**

  * **Vấn đề cần giải quyết:** Phải tái kiến trúc `Scheduler` và mô hình thực thi `System` để chúng trở nên linh hoạt, cho phép mỗi `System` có thể yêu cầu một tập hợp `Resource` khác nhau.
  * **Mục tiêu tiếp theo:** Không phải là thêm tính năng, mà là một cuộc **tái cấu trúc kiến trúc (architectural refactoring)** quan trọng để sửa lỗi build và làm cho nền tảng vững chắc hơn cho tương lai. Chúng ta sẽ chuyển từ mô hình hàm con trỏ cố định sang mô hình dựa trên `trait`.
  * **Hành động cụ thể:**
    1.  Định nghĩa một `System` `trait` chung.
    2.  Biến mỗi hàm `system` hiện tại thành một `struct` rỗng và `impl` `trait` đó cho nó.
    3.  Tạo một `Resource` container để chứa tất cả các tài nguyên toàn cục (`Mouse`, `Keyboard`, v.v.).
    4.  Nâng cấp `Scheduler` để nó có thể thực thi các `System` đa dạng thông qua `trait` này.

**3. Hình thành Nhiệm vụ Mới (Formulating the New Task):**

  * **ID Nhiệm vụ:** `T-250704-12`
  * **Tên Nhiệm vụ:** "Tái kiến trúc Bộ điều phối (Scheduler) để Hỗ trợ Hệ thống Đa dạng" (Refactor the Scheduler to Support Diverse Systems).
  * **Trọng tâm:** Sửa lỗi build bằng cách tái cấu trúc `Scheduler` và các `System`, làm cho kiến trúc trở nên linh hoạt và mạnh mẽ hơn.

-----

### **TẠO FILE TASK TIẾP THEO**

Dưới đây là nội dung của file `TASK_20250704_ENHANCE_PLATFORM_V12.md` sẽ được tạo ra và giao cho Coder.

````markdown
# YÊU CẦU TRIỂN KHAI: TÁI KIẾN TRÚC BỘ ĐIỀU PHỐI HỆ THỐNG V2.1

**TỪ:** Guardian (Kiến trúc sư Hệ thống)
**GỬI:** Coder (Lập trình viên Triển khai)
**NGÀY:** 04-07-2025
**ID NHIỆM VỤ:** T-250704-12

---

### 1. MỤC TIÊU TỔNG QUAN

**Cảnh báo: Ưu tiên Cao nhất.** Hệ thống hiện tại không thể biên dịch được do lỗi không tương thích kiểu trong `Scheduler`.

Nhiệm vụ này là một cuộc tái cấu trúc kiến trúc khẩn cấp và quan trọng. Chúng ta sẽ không thêm tính năng mới. Thay vào đó, chúng ta sẽ **sửa lỗi build và nâng cấp `Scheduler`** để nó có thể xử lý các `System` với các yêu cầu tài nguyên (Resource) đa dạng, làm cho nền tảng thực sự linh hoạt và sẵn sàng cho mở rộng.

---

### 2. BỐI CẢNH

* **Nguyên nhân lỗi:** `Scheduler` của chúng ta quá đơn giản, chỉ chấp nhận các `System` có chữ ký `fn(&mut World, &Mouse)`. Khi `System` `interact` cần thêm `Resource` `Keyboard`, kiến trúc cũ đã bị phá vỡ.
* **Giải pháp:** Chúng ta sẽ chuyển sang một mô hình hướng `trait` mạnh mẽ hơn, một kỹ thuật phổ biến trong các framework ECS thực thụ.

---

### 3. YÊU CẦU NHIỆM VỤ CHI TIẾT

#### 3.1. Cập nhật `pkb/todo.csv`

Thêm các công việc mới sau vào cuối file `pkb/todo.csv`:

```csv
T055,"(Refactor) Định nghĩa System Trait và Resource Container",Coder,Open,Critical
T056,"(Refactor) Chuyển đổi tất cả các system hiện tại thành các struct rỗng",Coder,Open,Critical
T057,"(Refactor) Triển khai System Trait cho tất cả các struct system",Coder,Open,Critical
T058,"(Refactor) Nâng cấp Scheduler để thực thi các Box<dyn System>",Coder,Open,Critical
T059,"(Refactor) Cập nhật main.rs và xác minh build thành công",Coder,Open,Critical
T060,"(Refactor) Viết báo cáo triển khai V12 và cập nhật PKB",Coder,Open,High
````

#### 3.2. Định nghĩa `System` Trait và `Resource` Container

  * **File:** `src/main.rs` hoặc một module mới `src/engine.rs`.
  * **Nhiệm vụ:**
    1.  Tạo một `struct` `Resources` để chứa tất cả các tài nguyên toàn cục.
        ```rust
        // src/resources/mod.rs
        pub struct Resources {
            pub mouse: Mouse,
            pub keyboard: Keyboard,
            // ... các resource khác trong tương lai
        }
        ```
    2.  Định nghĩa một `trait` `System`.
        ```rust
        // src/engine.rs
        pub trait System {
            // Mỗi system sẽ tự lấy resource nó cần từ `Resources`.
            fn run(&mut self, world: &mut World, resources: &mut Resources);
        }
        ```

#### 3.3. Tái cấu trúc các `System`

  * **Files:** Tất cả các file trong `src/systems/`.
  * **Nhiệm vụ:** Với **mỗi** `System` hiện có (`interact`, `layout`, `render`, v.v.):
    1.  Chuyển hàm `pub fn` thành một `struct` rỗng. Ví dụ: `pub fn interact(...)` -\> `pub struct Interact;`.
    2.  Triển khai `trait System` cho `struct` đó.
    3.  Di chuyển toàn bộ logic của hàm cũ vào bên trong phương thức `run`.
    4.  Bên trong `run`, lấy các `Resource` cần thiết từ tham số `resources`.
    <!-- end list -->
      * **Ví dụ cho `interact`:**
        ```rust
        // src/systems/interaction.rs
        pub struct Interact;
        impl System for Interact {
            fn run(&mut self, world: &mut World, resources: &mut Resources) {
                // Lấy mouse và keyboard từ resource container.
                let mouse = &resources.mouse;
                let keyboard = &resources.keyboard;
                // ... logic cũ của interact ...
            }
        }
        ```

#### 3.4. Nâng cấp `Scheduler`

  * **File:** `src/main.rs` hoặc `src/engine.rs`.
  * **Nhiệm vụ:**
    1.  Thay đổi trường `systems` của `Scheduler` từ `Vec<fn(...)>` thành `Vec<Box<dyn System>>`.
    2.  Cập nhật phương thức `run` của `Scheduler` để nó lặp qua `Vec` mới và gọi `system.run(world, resources)`.
    3.  Cập nhật phương thức `add` để nhận `Box<dyn System>`.

#### 3.5. Cập nhật `App` và `main.rs`

  * **File:** `src/main.rs`.
  * **Nhiệm vụ:**
    1.  Trong `struct App`, thay thế các `Resource` riêng lẻ (`mouse`, `keyboard`) bằng một `struct` `Resources` duy nhất.
    2.  Trong hàm `initialize`, khi thêm `System` vào `Scheduler`, hãy khởi tạo các `struct` và `Box` chúng.
        ```rust
        // Thay vì: scheduler.add(interaction::interact);
        // Bây giờ là:
        scheduler.add(Box::new(Interact));
        scheduler.add(Box::new(Layout));
        // ...
        ```
    3.  Cập nhật vòng lặp `run` của `App` để truyền `&mut self.world` và `&mut self.resources` vào `scheduler.run`.

#### 3.6. Xác minh Build

  * **Nhiệm vụ:** Sau khi hoàn thành tất cả các bước trên, hãy chạy `cargo build` và `cargo clippy`. **Mục tiêu là phải không còn bất kỳ lỗi (error) hay cảnh báo (warning) nào.**

-----

### 4\. ĐỊNH DẠNG PHẢN HỒI (BẮT BUỘC)

1.  Tạo file mới: `IMPLEMENTATION_REPORT_V12.md`.
2.  Sử dụng mẫu báo cáo đã cung cấp, cập nhật ID Nhiệm vụ thành `T-250704-12`.
3.  Trong mục **"Các Vấn đề hoặc Câu hỏi"**, hãy xác nhận rằng codebase đã biên dịch thành công và `clippy` không còn cảnh báo nào.

-----

### 5\. QUY TRÌNH NỘP BÀI

Thực hiện các lệnh Git sau:

```bash
git add .
git commit -m "refactor(engine)!: implement trait-based scheduler for diverse systems" -m "Fulfills task T-250704-12. Fixed critical build error by refactoring the scheduler and systems to a trait-based model. The engine is now more flexible and robust."
git push origin main
```

```
```