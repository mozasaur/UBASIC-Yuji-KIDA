;CALL.ASM

	INCLUDE	UBDEF.H
	INCLUDE	UB.H

	;UBASIC user make rourine 用 runtime package


DATA	SEGMENT	WORD PUBLIC

	public	baseoff

	EXTRN	CALCSP:WORD,HANDLE:WORD,FNAMEBUF:BYTE,MAXALL:WORD
	EXTRN	POINTWORD:WORD

FUNC_TBL2	DW	UFERR
	DW	SGN_ENT,LEN_ENT,BIT_ENT,GCD_ENT,SQRT_ENT
	DW	ISQRT_ENT,FIX_ENT,KRO_ENT,SIN_ENT,COS_ENT
	DW	EXP_ENT,LOG_ENT,PAI_ENT,PRM_ENT,MOB_ENT
	DW	EUL_ENT,PRMDIV_ENT,UFERR,UFERR,UFERR
	DW	UFERR,UFERR,UFERR,UFERR,UFERR
	DW	MODPOW_ENT,MODINV_ENT,INT_ENT,ASCLEN_ENT,NXTPRM_ENT
	DW	MAX_ENT,MIN_ENT,SHIFT_ENT,UFERR,UFERR


BASEOFF	DW	?
BASESEG	DW	?

DATA	ENDS


CODE	SEGMENT	WORD PUBLIC
	ASSUME	CS:CODE,DS:DATA



SYSCONST	EQU	40H

	PUBLIC	SYSPUSH,SYSPOP,SYSPUSHAX,UNDEF,UNDEF,UNDEF,UNDEF,UNDEF
	PUBLIC	SPADD,SPSUB,SPMUL,SPDIV,SPIDIV,SPMOD,SP2P,UNDEF
	PUBLIC	UNDEF
	PUBLIC	CALLIN,USRLOADIN,USRFUNC

	extrn	callusr:near
	EXTRN	ADDIN:NEAR,SUBIN:NEAR,MULIN:NEAR
	EXTRN	DIVIN:NEAR,IDIVIN:NEAR,RESIN:NEAR,POWER2IN:NEAR
	EXTRN	GETVADR:NEAR,MAINLP:NEAR
	EXTRN	farSETFNAME:far,EXTUBB:NEAR

	extrn	CloseHandle:far

	EXTRN	SGN_ENT:NEAR,LEN_ENT:NEAR,BIT_ENT:NEAR,GCD_ENT:NEAR
	EXTRN	SQRT_ENT:NEAR,ISQRT_ENT:NEAR,FIX_ENT:NEAR,KRO_ENT:NEAR
	EXTRN	SIN_ENT:NEAR,COS_ENT:NEAR,EXP_ENT:NEAR,LOG_ENT:NEAR
	EXTRN	PAI_ENT:NEAR,PRM_ENT:NEAR,MOB_ENT:NEAR,EUL_ENT:NEAR
	EXTRN	PRMDIV_ENT:NEAR,MODPOW_ENT:NEAR,MODINV_ENT:NEAR
	EXTRN	INT_ENT:NEAR,ASCLEN_ENT:NEAR,NXTPRM_ENT:NEAR
	EXTRN	MAX_ENT:NEAR,MIN_ENT:NEAR,SHIFT_ENT:NEAR

	EXTRN	SYNERR:NEAR,UNDEFERR:NEAR,USRERR:NEAR
	EXTRN	OPENERR:NEAR,DISKERR:NEAR,USRBIGERR:NEAR


;
;	関数の計算
;

USRFUNC	PROC FAR

	MOV	BX,OFFSET FUNC_TBL2
	XOR	AH,AH
	SHL	AX,1
	ADD	BX,AX
	CALL	SS:[BX]
	RET			;FAR RETURN

USRFUNC	ENDP



UFERR	PROC	FAR		;USER FUNCTION CALL
	STC
	RET
UFERR	ENDP

;
;	ユーザー作成機械語ルーチンをロードする
;	
	
USRLOADIN:

	;PATH 名をセット

	CALL	far ptr farSETFNAME	;PATH 名を得る
	jmpc	openerr
	CALL	EXTUBB		;EXTENSION を .UBB に

	;ファイルのOPEN

	MOV	AH,3DH		;OPEN HANDLE
	MOV	AL,0		;FOR READ
	MOV	DX,OFFSET FNAMEBUF
	INT	21H
	JMPC	OPENERR		;can't open

	MOV	[HANDLE],AX

	;配列名の取得

	cmp	byte ptr [bp],0c2h	;code of ,
	JNE	USRSYNERR
	INC	BP

	cmp	byte ptr [bp],VCODE1
	JNE	USRSYNERR	;短配列でない場合

	INC	BP

	xor	ax,ax
	mov	al,[bp]
	inc	bp
	mov	bx,[bp]
	add	bp,ax
	MOV	AX,cs:[BX]
	or	ax,ax
	JZ	USRUNDEF	;未定義配列
	MOV	[BASESEG],AX

	;SEQUENTIAL な読み込み

	MOV	DS,AX
	CALL	GETARRAYSIZE
	SUB	CX,100H		;HEADER の分
	JBE	BIG_FILE
	MOV	AH,3FH		;read
	MOV	BX,SS:[HANDLE]
	MOV	DX,100H
	INT	21H
	MOV	DX,SS
	MOV	DS,DX
	PUSHF
	CALL	far ptr CloseHandle
	POPF
	JNC	USRLOADOK

	JMP	DISKERR
USRUNDEF:
	JMP	UNDEFERR
USRSYNERR:
	JMP	SYNERR
BIG_FILE:
	JMP	USRBIGERR


USRLOADOK:
	CMP	AX,CX
	JAE	BIG_FILE

	;配列１つの BYTE 数をSET

	MOV	ES,[BASESEG]
	MOV	DI,SYSCONST
	MOV	AX,SS:[MAXALL]
	SHL	AX,1		;長変数
	STOSW			;SYSCONST番地

	MOV	AX,UNITBYTE	;特別変数
	STOSW			;SYSCONST+2番地

	;STACK POINTER の位置をセット

	MOV	AX,OFFSET CALCSP
	STOSW			;SYSCONST+4番地

	;POINT のセット

	MOV	AX,SS:[POINTWORD]
	STOSW			;SYSCONST+6番地

	;set UBCONSTSEG

  if flg98
	mov	ax,UBCONSTSEG
  else
	mov	ax,ss
	add	ax,ubconstseg
  endif
	stosw			;SYSCONST+8

	;各引数の SET

	MOV	AX,SS
	MOV	DS,AX
	MOV	[BASEOFF],80H
USRLP:
	MOV	AL,[BP]
	CMP	AL,')'
	JE	USRJP		;引数がなくなった
	cmp	al,CR
	je	usrjp10
	cmp	al,0c0h		;':'
	je	usrjp

	CMP	AL,0C2H		;code of ','
	JNE	USR20
	INC	BP
	ADD	SS:[BASEOFF],4
	JMP	USRLP
USR20:
	CALL	GETVADR		;DS:SI が返る
	LES	DI,DWORD PTR SS:[BASEOFF]
	MOV	AX,SI
	STOSW
	MOV	AX,DS
	STOSW

	MOV	AX,SS
	MOV	DS,AX
	MOV	ES,AX
	MOV	[BASEOFF],DI
	MOV	AL,[BP]
	CMP	AL,')'
	JE	USRJP		;引数がなくなった
	CMP	AL,0C2H		;code of ','
	JNZ	USRSYNERR
	INC	BP
	JMP	USRLP

USRJP:
	INC	BP
usrjp10:
	MOV	AX,SS
	MOV	DS,AX
	MOV	ES,AX
	JMP	MAINLP



GETARRAYSIZE:
	MOV	CX,0FE00H
	XOR	SI,SI
	MOV	AX,[SI+4]	;サイズ
	CMP	AX,7F00H
	JAE	GASIZERET
	SHL	AX,1
	ADD	AX,ARRAYHEAD
	MOV	CX,AX
GASIZERET:
	RET


;
;	ユーザー作成機械語ルーチンをコールする
;

CALLSYNERR:
	JMP	SYNERR
CALLUNDEF:
	JMP	UNDEFERR
CALLILEG:
	JMP	USRERR

CALLIN:
	cmp	byte ptr [bp],VCODE1
	JNE	CALLSYNERR	;短配列でない場合

	INC	BP
	xor	ax,ax
	mov	al,[bp]
	inc	bp
	mov	bx,[bp]
	add	bp,ax
	MOV	AX,cs:[BX]
	or	ax,ax
	JZ	CALLUNDEF	;未定義配列

	MOV	[BASESEG],AX
	MOV	[BASEOFF],80H
	MOV	DS,AX
	MOV	BX,SYSCONST
	MOV	AX,[BX]
	SHR	AX,1
	CMP	AX,SS:[MAXALL]
	JNE	CALLILEG	;SYSTEM CALL が SET されていない

	;各引数の SET

	MOV	AX,SS
	MOV	DS,AX
	MOV	[BASEOFF],80H
CALLLP:
	MOV	AL,[BP]
	CMP	AL,')'
	JE	CALLJP		;引数がなくなった
	cmp	al,CR
	je	calljp10
	cmp	al,0c0h		;':'
	je	calljp

	CMP	AL,0C2H		;code of ','
	JNE	CALL20
	INC	BP
	ADD	SS:[BASEOFF],4
	JMP	CALLLP

CALLJP:
	INC	BP
calljp10:
;//	PUSH	BP
	MOV	SS:[BASEOFF],100H
	jmp	CALLUSR

CALL20:
	CALL	GETVADR		;DS:SI が返る
	LES	DI,DWORD PTR SS:[BASEOFF]
	MOV	AX,SI
	STOSW
	MOV	AX,DS
	STOSW
	MOV	AX,SS
	MOV	DS,AX
	MOV	[BASEOFF],DI
	MOV	AL,[BP]
	CMP	AL,')'
	JE	CALLJP		;引数がなくなった
	CMP	AL,0C2H		;code of ','
	JNE	callSYNERR2
	INC	BP
	JMP	CALLLP

callsynerr2:
	jmp	synerr


;
;	ユーザー用ユーティリティパッケージ
;

UNDEF	PROC FAR
	RET
UNDEF	ENDP


SPADD	PROC FAR

; [SP2]=[SP2]+[SP1],SP1=SP2

	MOV	AX,SS
	MOV	DS,AX
	MOV	ES,AX
	CALL	ADDIN
	RET			;FAR RET

SPADD	ENDP


SPSUB	PROC FAR

; [SP2]=[SP2]-[SP1],SP1=SP2

	MOV	AX,SS
	MOV	DS,AX
	MOV	ES,AX
	CALL	SUBIN
	RET			;FAR RET

SPSUB	ENDP


SPMUL	PROC FAR

; [SP2]=[SP2]*[SP1],SP1=SP2

	MOV	AX,SS
	MOV	DS,AX
	MOV	ES,AX
	CALL	MULIN
	RET			;FAR RET

SPMUL	ENDP


SPDIV	PROC FAR

; [SP2]=[SP2]/[SP1],SP1=SP2

	MOV	AX,SS
	MOV	DS,AX
	MOV	ES,AX
	CALL	DIVIN
	RET			;FAR RET

SPDIV	ENDP


SPIDIV	PROC FAR

; [SP2]=[SP2]\[SP1],SP1=SP2
; [RESADR]=余り

	MOV	AX,SS
	MOV	DS,AX
	MOV	ES,AX
	CALL	IDIVIN
	RET			;FAR RET

SPIDIV	ENDP


SPMOD	PROC FAR

; [SP2]=[SP2]@[SP1],SP1=SP2

	MOV	AX,SS
	MOV	DS,AX
	MOV	ES,AX
	CALL	RESIN
	RET			;FAR RET

SPMOD	ENDP


SP2P	PROC FAR

; [SP1]=[SP1]^2

	MOV	AX,SS
	MOV	DS,AX
	MOV	ES,AX
	CALL	POWER2IN
	RET			;FAR RET

SP2P	ENDP


SYSPUSH	PROC FAR

; [DS:SI] を計算スタックに積む

	MOV	AX,SS
	MOV	ES,AX

	MOV	AX,SS:[CALCSP]
	SUB	AX,UNITBYTE	;STACK OVERFLOW は NON CHECK
	MOV	SS:[CALCSP],AX
	MOV	DI,AX

	MOV	CX,[SI]
	AND	CX,LENMASK
	INC	CX
	REP	MOVSW

	RET			;FAR RET

SYSPUSH	ENDP


SYSPUSHAX	PROC FAR

; 0<=AX<=65535 を計算スタックに積む

	MOV	BX,SS
	MOV	DS,BX

	MOV	BX,SS:[CALCSP]
	SUB	BX,UNITBYTE	;STACK OVERFLOW は NON CHECK
	MOV	SS:[CALCSP],BX

	OR	AX,AX
	JZ	SHORT JP1
	MOV	WORD PTR [BX],1
	ADD	BX,2
JP1:
	MOV	[BX],AX
	RET			;FAR RET

SYSPUSHAX	ENDP


SYSPOP	PROC FAR

; 計算スタックから [ES:DI] へ

	MOV	AX,SS
	MOV	DS,AX

	MOV	AX,[CALCSP]
	MOV	SI,AX
	ADD	AX,UNITBYTE
	MOV	[CALCSP],AX

	MOV	CX,[SI]
	AND	CX,LENMASK
	INC	CX
	REP	MOVSW

	RET			;FAR RET

SYSPOP	ENDP

		
CODE	ENDS
END
