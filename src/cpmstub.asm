  .org $dc00
bdos:
  mov b, a
  mov a, c
  cpi 2
  jz conout
  cpi 9
  jz prstring
  cpi 0
  jz reset
  ret

conout:
  mov a, b
  out $aa
  ret

prstring:
  ldax d
  cpi '$'
  rz
  inx d
  mov b, a
  call conout
  jmp prstring

reset:
  out $ff
  ret
