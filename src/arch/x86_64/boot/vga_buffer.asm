%define VGA_BUFFER 0xb8000
%define VGA_WIDTH 80
%define VGA_HEIGHT 25
%define VGA_CHAR_SIZE 2
%define VGA_MAX_CHARS (VGA_WIDTH * VGA_HEIGHT)

%define VGA_POSITION(X, Y)  (VGA_BUFFER + ((X) + (Y) * VGA_WIDTH) * VGA_CHAR_SIZE)