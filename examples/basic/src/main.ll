; ModuleID = 'koa_module'
source_filename = "koa_module"

@str_40 = unnamed_addr constant [41 x i8] c"========================================\00"
@str_17 = unnamed_addr constant [18 x i8] c"Koa Language Demo\00"
@str_40.1 = unnamed_addr constant [41 x i8] c"========================================\00"
@str_0 = unnamed_addr constant [1 x i8] zeroinitializer
@str_18 = unnamed_addr constant [19 x i8] c"--- Arithmetic ---\00"
@str_14 = unnamed_addr constant [15 x i8] c"%d + %d = %d\\n\00"
@str_14.2 = unnamed_addr constant [15 x i8] c"%d * %d = %d\\n\00"
@str_0.3 = unnamed_addr constant [1 x i8] zeroinitializer
@str_17.4 = unnamed_addr constant [18 x i8] c"--- Functions ---\00"
@str_19 = unnamed_addr constant [20 x i8] c"factorial(5) = %d\\n\00"
@str_20 = unnamed_addr constant [21 x i8] c"fibonacci(10) = %d\\n\00"
@str_0.5 = unnamed_addr constant [1 x i8] zeroinitializer
@str_40.6 = unnamed_addr constant [41 x i8] c"========================================\00"
@str_14.7 = unnamed_addr constant [15 x i8] c"Demo Complete!\00"
@str_40.8 = unnamed_addr constant [41 x i8] c"========================================\00"
@str_0.9 = unnamed_addr constant [1 x i8] zeroinitializer

declare i32 @printf(ptr, ...)

declare i32 @puts(ptr)

define i32 @add(i32 %0, i32 %1) {
entry_0:
  %x = alloca i32, align 4
  store i32 %0, ptr %x, align 4
  %y = alloca i32, align 4
  store i32 %1, ptr %y, align 4
  %load_x = load i32, ptr %x, align 4
  %load_y = load i32, ptr %y, align 4
  %t0 = add i32 %load_x, %load_y
  ret i32 %t0

entry_01:                                         ; No predecessors!
  %x2 = alloca i32, align 4
  store i32 %0, ptr %x2, align 4
  %y3 = alloca i32, align 4
  store i32 %1, ptr %y3, align 4
  %load_x4 = load i32, ptr %x2, align 4
  %load_y5 = load i32, ptr %y3, align 4
  %t06 = add i32 %load_x4, %load_y5
  ret i32 %t06
}

define i32 @mul(i32 %0, i32 %1) {
entry_0:
  %x = alloca i32, align 4
  store i32 %0, ptr %x, align 4
  %y = alloca i32, align 4
  store i32 %1, ptr %y, align 4
  %load_x = load i32, ptr %x, align 4
  %load_y = load i32, ptr %y, align 4
  %t0 = mul i32 %load_x, %load_y
  ret i32 %t0

entry_01:                                         ; No predecessors!
  %x2 = alloca i32, align 4
  store i32 %0, ptr %x2, align 4
  %y3 = alloca i32, align 4
  store i32 %1, ptr %y3, align 4
  %load_x4 = load i32, ptr %x2, align 4
  %load_y5 = load i32, ptr %y3, align 4
  %t06 = mul i32 %load_x4, %load_y5
  ret i32 %t06
}

define i32 @factorial(i32 %0) {
entry_0:
  %n = alloca i32, align 4
  store i32 %0, ptr %n, align 4
  %load_n = load i32, ptr %n, align 4
  %t0 = icmp sle i32 %load_n, 1
  br i1 %t0, label %if_then_1, label %if_merge_1

if_then_1:                                        ; preds = %entry_0
  ret i32 1
  br label %if_merge_1

if_merge_1:                                       ; preds = %if_then_1, %entry_0
  %load_n1 = load i32, ptr %n, align 4
  %t1 = sub i32 %load_n1, 1
  %call = call i32 @factorial(i32 %t1)
  %load_n2 = load i32, ptr %n, align 4
  %t3 = mul i32 %load_n2, %call
  ret i32 %t3

entry_03:                                         ; No predecessors!
  %n6 = alloca i32, align 4
  store i32 %0, ptr %n6, align 4
  %load_n7 = load i32, ptr %n6, align 4
  %t08 = icmp sle i32 %load_n7, 1
  br i1 %t08, label %if_then_14, label %if_merge_15

if_then_14:                                       ; preds = %entry_03
  ret i32 1
  br label %if_merge_15

if_merge_15:                                      ; preds = %if_then_14, %entry_03
  %load_n9 = load i32, ptr %n6, align 4
  %t110 = sub i32 %load_n9, 1
  %call11 = call i32 @factorial(i32 %t110)
  %load_n12 = load i32, ptr %n6, align 4
  %t313 = mul i32 %load_n12, %call11
  ret i32 %t313
}

define i32 @fibonacci(i32 %0) {
entry_0:
  %n = alloca i32, align 4
  store i32 %0, ptr %n, align 4
  %load_n = load i32, ptr %n, align 4
  %t0 = icmp sle i32 %load_n, 1
  br i1 %t0, label %if_then_1, label %if_merge_1

if_then_1:                                        ; preds = %entry_0
  %load_n1 = load i32, ptr %n, align 4
  ret i32 %load_n1
  br label %if_merge_1

if_merge_1:                                       ; preds = %if_then_1, %entry_0
  %load_n2 = load i32, ptr %n, align 4
  %t1 = sub i32 %load_n2, 1
  %call = call i32 @fibonacci(i32 %t1)
  %load_n3 = load i32, ptr %n, align 4
  %t3 = sub i32 %load_n3, 2
  %call4 = call i32 @fibonacci(i32 %t3)
  %t5 = add i32 %call, %call4
  ret i32 %t5

entry_05:                                         ; No predecessors!
  %n8 = alloca i32, align 4
  store i32 %0, ptr %n8, align 4
  %load_n9 = load i32, ptr %n8, align 4
  %t010 = icmp sle i32 %load_n9, 1
  br i1 %t010, label %if_then_16, label %if_merge_17

if_then_16:                                       ; preds = %entry_05
  %load_n11 = load i32, ptr %n8, align 4
  ret i32 %load_n11
  br label %if_merge_17

if_merge_17:                                      ; preds = %if_then_16, %entry_05
  %load_n12 = load i32, ptr %n8, align 4
  %t113 = sub i32 %load_n12, 1
  %call14 = call i32 @fibonacci(i32 %t113)
  %load_n15 = load i32, ptr %n8, align 4
  %t316 = sub i32 %load_n15, 2
  %call17 = call i32 @fibonacci(i32 %t316)
  %t518 = add i32 %call14, %call17
  ret i32 %t518
}

define i32 @main() {
entry_0:
  %call = call i32 @puts(ptr @str_40)
  %call1 = call i32 @puts(ptr @str_17)
  %call2 = call i32 @puts(ptr @str_40.1)
  %call3 = call i32 @puts(ptr @str_0)
  %call4 = call i32 @puts(ptr @str_18)
  %a = alloca i32, align 4
  store i32 10, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 5, ptr %b, align 4
  %load_a = load i32, ptr %a, align 4
  %load_b = load i32, ptr %b, align 4
  %call5 = call i32 @add(i32 %load_a, i32 %load_b)
  %load_a6 = load i32, ptr %a, align 4
  %load_b7 = load i32, ptr %b, align 4
  %call8 = call i32 (ptr, ...) @printf(ptr @str_14, i32 %load_a6, i32 %load_b7, i32 %call5)
  %load_a9 = load i32, ptr %a, align 4
  %load_b10 = load i32, ptr %b, align 4
  %call11 = call i32 @mul(i32 %load_a9, i32 %load_b10)
  %load_a12 = load i32, ptr %a, align 4
  %load_b13 = load i32, ptr %b, align 4
  %call14 = call i32 (ptr, ...) @printf(ptr @str_14.2, i32 %load_a12, i32 %load_b13, i32 %call11)
  %call15 = call i32 @puts(ptr @str_0.3)
  %call16 = call i32 @puts(ptr @str_17.4)
  %call17 = call i32 @factorial(i32 5)
  %call18 = call i32 (ptr, ...) @printf(ptr @str_19, i32 %call17)
  %call19 = call i32 @fibonacci(i32 10)
  %call20 = call i32 (ptr, ...) @printf(ptr @str_20, i32 %call19)
  %call21 = call i32 @puts(ptr @str_0.5)
  %call22 = call i32 @puts(ptr @str_40.6)
  %call23 = call i32 @puts(ptr @str_14.7)
  %call24 = call i32 @puts(ptr @str_40.8)
  %call25 = call i32 @puts(ptr @str_0.9)
  ret i32 0
}
