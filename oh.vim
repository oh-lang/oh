" Vim syntax file for oh-lang
" Add to `vimXX/filetype.vim`: `au BufNewFile,BufRead *.oh	setf oh`
" Add this file to the `vimXX/syntax/` directory.
" Language:	oh
" Last Change:	2025 Apr 14
" Credits:	Zvezdan Petkovic <zpetkovic@acm.org>
"		Neil Schemenauer <nas@python.ca>
"		Dmitry Vasiliev

" quit when a syntax file was already loaded.
if exists("b:current_syntax")
  finish
endif

" We need nocompatible mode in order to continue lines with backslashes.
" Original setting will be restored.
let s:cpo_save = &cpo
set cpo&vim

" Keep keywords in alphabetical order inside groups.
"
syn keyword ohStatement		false null true
syn keyword ohStatement		break continue
syn keyword ohStatement		pass return
syn keyword ohConditional	elif else if
syn keyword ohConditional	what
syn keyword ohRepeat		each each_ while
syn keyword ohOperator		and is is_ not or xor
syn keyword ohError		er
syn keyword ohAsync		decide_ um um_
syn keyword ohTodo		FIXME NOTE NOTES TODO XXX contained

syn region	ohInclude matchgroup=ohInclude start="\\\\" end=" " skip="\\ "
syn region	ohInclude matchgroup=ohInclude start="\\/" end=" " skip="\\ "
syn match	ohUnused	"\<_\w*"
syn match	ohMacro		"@\w*\>"
syn match	ohNamespace	"\u\+_\="
syn match	ohFunction	"\h\w*_\>" contains=ohNamespace

" TODO: midline comment
syn match   ohComment	"#.*$" contains=ohTodo,@Spell
syn region  ohMultilineComment matchgroup=ohMultilineComment
      \ start=+#(#+ end=+#)#+ keepend
      \ contains=ohEscape,@Spell
syn region  ohMultilineComment matchgroup=ohMultilineComment
      \ start=+#\[#+ end=+#\]#+ keepend
      \ contains=ohEscape,@Spell
syn region  ohMultilineComment matchgroup=ohMultilineComment
      \ start=+#{#+ end=+#}#+ keepend
      \ contains=ohEscape,@Spell

syn region  ohString matchgroup=ohQuotes
      \ start=+\z(['"]\)+ end="\z1" skip="\\\\\|\\\z1"
      \ contains=ohEscape,@Spell

syn match   ohEscape	+\\[abfnrtv'"\\]+ contained
syn match   ohEscape	"\\\o\{1,3}" contained
syn match   ohEscape	"\\x\x\{2}" contained
syn match   ohEscape	"\%(\\u\x\{4}\|\\U\x\{8}\)" contained
" Python allows case-insensitive Unicode IDs: http://www.unicode.org/charts/
syn match   ohEscape	"\\N{\a\+\%(\s\a\+\)*}" contained
" TODO: we need ${}, $(), and $[] in strings.

syn match   ohNumber	"\<0[oO]\=\o\+[Ll]\=\>"
syn match   ohNumber	"\<0[xX]\x\+[Ll]\=\>"
syn match   ohNumber	"\<0[bB][01]\+[Ll]\=\>"
syn match   ohNumber	"\<\%([1-9]\d*\|0\)[Ll]\=\>"
syn match   ohNumber	"\<\d\+[jJ]\>"
syn match   ohNumber	"\<\d\+[eE][+-]\=\d\+[jJ]\=\>"
syn match   ohNumber	"\<\d\+\.\%([eE][+-]\=\d\+\)\=[jJ]\=\%(\W\|$\)\@="
syn match   ohNumber	"\%(^\|\W\)\zs\d*\.\d\+\%([eE][+-]\=\d\+\)\=[jJ]\=\>"

syn keyword ohBuiltin	false true null null_
syn keyword ohBuiltin	abs_ ceil_ floor_
syn keyword ohBuiltin	error_ print_

" trailing whitespace
syn match   ohSpaceError	display excludenl "\s\+$"
" mixed tabs and spaces
syn match   ohSpaceError	display " \+\t"
syn match   ohSpaceError	display "\t\+ "

" The default highlight links.  Can be overridden later.
hi def link ohNamespace		Comment
hi def link ohStatement		Statement
hi def link ohConditional	Conditional
hi def link ohRepeat		Repeat
hi def link ohOperator		Operator
hi def link ohError		Exception
hi def link ohInclude		Include
hi def link ohAsync		Statement
hi def link ohBuiltin		Function
hi def link ohFunction		Function
hi def link ohUnused	Include
hi def link ohComment		Comment
hi def link ohTodo		Todo
hi def link ohString		String
hi def link ohQuotes		String
hi def link ohMultilineComment	ohComment
hi def link ohEscape		Special
hi def link ohNumber		Number
hi def link ohSpaceError	Error
hi def link ohMacro		Identifier
" TODO: make [] be a `Type`

let b:current_syntax = "oh"

let &cpo = s:cpo_save
unlet s:cpo_save

" vim:set sw=4 et:
