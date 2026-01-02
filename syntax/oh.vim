" Vim syntax file for oh-lang
" Add to `vimXX/filetype.vim`: `au BufNewFile,BufRead *.oh	setf oh`
" Add this file to the `vimXX/syntax/` directory.
" Language:	oh
" Last Change:	2025 July 30
" Credits:	Zvezdan Petkovic <zpetkovic@acm.org>
"		Neil Schemenauer <nas@python.ca>
"		Dmitry Vasiliev

" quit when a syntax file was already loaded.
if exists("b:current_syntax")
  finish
endif

set tabstop=5
set softtabstop=5
set shiftwidth=0

" We need nocompatible mode in order to continue lines with backslashes.
" Original setting will be restored.
let s:cpo_save = &cpo
set cpo&vim

" TODO: let's see if we can make the vim syntax work for ending in a number;
" i.e., it should become a namespace

" NOTE: in a group, lower elements have higher priority.
syn keyword ohBuiltinVariable	false true none null unspecified
syn keyword ohBuiltinVariable	ctx debug m o require
syn keyword ohBuiltinFunction	has is renew
syn keyword ohBuiltinFunction	count test to
syn keyword ohBuiltinFunction	abs ceil floor max min
syn keyword ohBuiltinFunction	error print
syn keyword ohStatement		break continue fall_through
syn keyword ohStatement		pass return
syn keyword ohJump	assert exit
syn keyword ohConditional	elif else if
syn keyword ohConditional	also what when where with
syn keyword ohRepeat		each while
syn keyword ohOperator		and has is not or xor
syn keyword ohError		er
syn keyword ohOk		ok
syn keyword ohResult		hm
syn keyword ohAsync		decide um #um
syn keyword ohTodo		FIXME NOTE NOTES TODO XXX contained
syn match	ohInferred	"\~[A-Za-z_][a-zA-Z_0-9]*"
syn match	ohReadonly	":"
syn match	ohWritable	";"
syn match	ohTemporary	"\."
syn match	ohNullSymbol	"?"
syn match	ohSyntaxError	"\$[0-9]*"
syn match	ohLambdaStarter	"\$[A-Za-z_][A-Za-z_0-9]*"
" Namespace syntax errors
" TODO: ASDF_D doesn't get captured for some reason.
syn match	ohSyntaxError	"\<[A-Z_]\+\>"

syn region	ohInclude matchgroup=ohInclude start="\\\\" end="[ \n]" skip="\\ "
syn region	ohInclude matchgroup=ohInclude start="\\/" end="[ \n]" skip="\\ "
syn match	ohUnused	"\<_[^ ()\[\]^`{|}!-@\\~]\+\>"
syn match	ohMacro		"@[^ ()\[\]{}@]*\>"
syn match	ohNamespace	"\u\+_"
syn match	ohNamespace	"_\u[A-Z_]*\>"
syn match	ohNamespace	"\zs\<\u\+\ze[^A-Z_ ]"
syn match	ohNamespace	"[^A-Z_ ]\zs\u\+\ze"
syn match	ohFunction	"\<_\>"
syn match	ohFunction	"[^_ ()\[\]^`{|}!-@\\~][^ ()\[\]^`{|}!-/:-@\\~]*_\>" contains=ohNamespace

syn region ohBraced matchgroup=ohBraces
      \ start=+{+ end="}"
      \ contains=ALL
syn region ohBracketed matchgroup=ohBrackets
      \ start=+\[+ end="]"
      \ contains=ALL
syn region ohParened matchgroup=ohParens
      \ start=+(+ end=")"
      \ contains=ALL

syn match ohType		"#[a-zA-Z][a-zA-Z0-9_]*"
syn match ohBuiltInVariable	"#o\>"
syn match ohBuiltInVariable	"#m\>"
syn match ohBuiltinType	"#never"
syn match ohBuiltinType "#disallowed"
syn match ohBuiltinType "#null"
syn match ohError	"#er"
syn match ohOk		"#ok"
syn match ohAsync	"#um"
syn match ohInferred	"\~#[a-zA-Z_][a-zA-Z_0-9]*"
syn match ohBuiltinType		"\~#\s"
syn match ohCompilerErrorComment	"#@!.*$"
syn match ohEndOfLineComment		" # .*$"
      \ contains=ohEscape,ohTodo,ohTick,@Spell
syn region  ohMidlineComment matchgroup=ohMidlineComment
      \ oneline display
      \ start=+#([^#]+ end="[^#])#"
      \ contains=ohEscape,ohTodo,ohTick,@Spell
syn region  ohMidlineComment matchgroup=ohMidlineComment
      \ oneline display
      \ start=+#\[[^#]+ end=+[^#]\]#+
      \ contains=ohEscape,ohTodo,ohTick,@Spell
syn region  ohMidlineComment matchgroup=ohMidlineComment
      \ oneline display
      \ start=+#{[^#]+ end=+[^#]}#+
      \ contains=ohEscape,ohTodo,ohTick,@Spell
syn region  ohMultilineComment matchgroup=ohMultilineComment
      \ start=+#(#+ end=+#)#+ keepend
      \ contains=ohEscape,ohTodo,ohTick,@Spell
syn region  ohMultilineComment matchgroup=ohMultilineComment
      \ start=+#\[#+ end=+#\]#+ keepend
      \ contains=ohEscape,ohTodo,ohTick,@Spell
syn region  ohMultilineComment matchgroup=ohMultilineComment
      \ start=+#{#+ end=+#}#+ keepend
      \ contains=ohEscape,ohTodo,ohTick,@Spell

syn region  ohString oneline matchgroup=ohQuotes
      \ start=+\z(['"]\)+ end="\z1" skip="\\\\\|\\\z1"
      \ contains=ohEscape,@Spell,ohStringInterpolation

syn match   ohEscape	+\\[abfnrtv'"\\]+ contained
syn match   ohEscape	"\\\o\{1,3}" contained
syn match   ohEscape	"\\x\x\{2}" contained
syn match   ohEscape	"\%(\\u\x\{4}\|\\U\x\{8}\)" contained
" Python allows case-insensitive Unicode IDs: http://www.unicode.org/charts/
syn match   ohEscape	"\\N{\a\+\%(\s\a\+\)*}" contained
" TODO: we need ${}, $(), and $[] in strings.

syn match   ohNumber	"\<0[oO][0-7][0-7_]*\>"
syn match   ohNumber	"\<0[xX]\x[0-9a-fA-F_]*\>"
syn match   ohNumber	"\<0[bB][01][01_]*\>"
syn match   ohNumber	"\<\d[0-9_]*\>"
syn match   ohNumber	"\<\d[0-9_]*\.[0-9_]*\>"
syn match   ohNumber	"\<\d[0-9_]*[eE][+-]\=\d[0-9_]*\>"
syn match   ohNumber	"\<\d[0-9_]*\.[0-9_]*[eE][+-]\=\d[0-9_]*\>"

" trailing whitespace
syn match   ohSpaceError	display excludenl "\s\+$"
" mixed tabs and spaces
syn match   ohSpaceError	display " \+\t"
syn match   ohSpaceError	display "\t\+ "

syn region	ohTick matchgroup=ohTick
      \ start=+`+ end=+`+
      \ contains=ALLBUT,ohTick
syn region ohStringInterpolation matchgroup=ohBraceInterpolation
      \ start=+${+ end=+}+ extend
      \ contains=ALLBUT,ohStringInterpolation
syn region ohStringInterpolation matchgroup=ohBracketInterpolation
      \ start=+$\[+ end=+]+ extend
      \ contains=ALLBUT,ohStringInterpolation
syn region ohStringInterpolation matchgroup=ohParenInterpolation
      \ start=+$(+ end=+)+ extend
      \ contains=ALLBUT,ohStringInterpolation

" The default highlight links.
" WildMenu is interesting.  see options with `:highlight`
hi def link ohNamespace		Folded
hi def link ohStatement		Statement
hi def link ohConditional	Conditional
hi def link ohRepeat		Repeat
hi def link ohOperator		Operator
hi def link ohError		WarningMsg
hi def link ohOk		CursorLine
hi def link ohResult		Folded
hi def link ohInclude		Include
hi def link ohInferred		Pmenu
hi def link ohAsync		Statement
hi def link ohBuiltinVariable		Title
hi def link ohBuiltinFunction		Question
hi def link ohBuiltinType		Removed
hi def link ohFunction		Function
hi def link ohType		Type
hi def link ohUnused	Comment
hi def link ohCompilerErrorComment		Error
hi def link ohEndOfLineComment		Comment
hi def link ohMultilineComment	Comment
hi def link ohMidlineComment	Comment
hi def link ohTodo		Todo
hi def link ohString		String
hi def link ohQuotes		String
hi def link ohEscape		Special
hi def link ohNumber		Number
hi def link ohSpaceError	Error
hi def link ohMacro		Identifier
hi def link ohJump		Identifier
hi def link ohLambdaStarter	Removed
hi def link ohNullSymbol		Identifier
hi def link ohBrackets		Type
hi def link ohBraces		NonText
hi def link ohParens		Operator
hi def link ohBraceInterpolation		ohBraces
hi def link ohBracketInterpolation		ohBrackets
hi def link ohParenInterpolation		ohParens
hi def link ohReadonly		Constant
hi def link ohWritable		Comment
hi def link ohTemporary		NonText
hi def link ohSyntaxError		Error

let b:current_syntax = "oh"

let &cpo = s:cpo_save
unlet s:cpo_save

" vim:set sw=4 et:
