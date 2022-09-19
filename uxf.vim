" Vim syntax file
" Language:        UXF
" Author:          Mark Summerfield <mark@qtrac.eu>
" URL:             https://github.com/mark-summerfield/uxf
" Licence:         Public Domain
" Latest Revision: 2022-09-19

if exists("b:current_syntax")
  finish
endif

syn clear
syn sync fromstart linebreaks=3 minlines=50

" Order matters!

syn keyword uxfTodo TODO FIXME DELETE CHECK TEST XXX
syn keyword uxfConst yes true no false
syn match uxfNull /[?]/
syn keyword uxfType bool int real date datetime str bytes list map table
syn match uxfPunctuation /[][{}()=:!]/
syn match uxfIdentifier /\<\w\+\>/ 
syn region uxfStr start="<" end=">"
syn region uxfComment start="#<" end=">" contains=uxfTodo keepend
syn match uxfBytes /(:[A-Fa-f0-9\s]\+:)/ contains=uxfIdentifier keepend
syn match uxfNumber /\<[-+]\=\d\+\(\.\d\+\([Ee][-+]\=\d\+\)\=\)\=\>/
syn match uxfDateTime /\<\d\d\d\d-\d\d-\d\d\(T\d\d\(:\d\d\(:\d\d\)\=\)\=\)\=\>/
syn match uxfHeader /^uxf\s*\d\+.*$/

" See https://sashamaps.net/docs/resources/20-colors/
hi uxfIdentifier guifg=#9A6324 "brown
hi uxfComment	guifg=#808000 term=italic cterm=italic gui=italic "olive
hi uxfStr  guifg=#469990 "teal
hi uxfConst guifg=#000075 "navy
hi uxfNull guifg=#E6194B "red
hi uxfBytes  guifg=#F58231 "orange
hi uxfTodo guibg=#FFE119 term=italic cterm=italic gui=italic "yellow
hi uxfDateTime guifg=#297B33 "green
hi uxfNumber  guifg=#4363D8 "blue
hi uxfType guifg=#911EB4 "purple
hi uxfPunctuation guifg=#F032E6 term=bold   cterm=bold   gui=bold "magenta
hi uxfHeader  guifg=navy guibg=#FFFAC8 "beige
