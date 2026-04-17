; ============================================
; Комментарии
; ============================================
(comment) @comment

; ============================================
; Строки
; ============================================
(string) @string

; ============================================
; Числа
; ============================================
(number) @number

; ============================================
; Ключевые слова (узлы)
; ============================================
(import) @keyword
(variable_definition) @keyword
(macro_definition) @keyword
(class_definition) @keyword
(if_statement) @keyword
(for_loop) @keyword
(while_loop) @keyword
(return_statement) @keyword
(constant_definition) @keyword
(with_definition) @keyword
(record_definition) @keyword

; ============================================
; Идентификаторы (все как переменные)
; ============================================
(identifier) @variable

; ============================================
; Определения макросов (имя функции)
; ============================================
(macro_definition
  name: (identifier) @function)

; ============================================
; Вызовы макросов
; ============================================
(macro_call
  name: (identifier) @function.call)

; ============================================
; Аннотации типов (Var var:ClassName)
; ============================================
(type_declaration
  (type) @type)

; ============================================
; Скалярные типы (integer, string, bool и т.д.)
; ============================================
(scalar_type) @type.builtin

; ============================================
; Объектные типы (clLogger, TRecHandler)
; ============================================
(object_type
  (identifier) @type)

; ============================================
; Логические значения
; ============================================
(boolean_literal) @constant.builtin

; ============================================
; Встроенные константы (V_INTEGER, V_STRING и т.д.)
; ============================================
(constant_builtin) @constant.builtin

; ============================================
; Специальные литералы (null, nullval, optval)
; ============================================
(special_literal) @constant.builtin

; ============================================
; Встроенные переменные (this, {curdate})
; ============================================
(variable_builtin) @variable.builtin

; ============================================
; Операторы
; ============================================
(binary_operator) @operator
(unary_operator) @operator
(assignment_operator) @operator

; ============================================
; Разделители
; ============================================
";" @punctuation.delimiter
"," @punctuation.delimiter
"." @punctuation.delimiter
":" @punctuation.delimiter

; ============================================
; Скобки
; ============================================
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket