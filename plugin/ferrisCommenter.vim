" Initialize the channel
if !exists('s:ferrisJobId')
	let s:ferrisJobId = 0
endif

" Constants for RPC messages.
let s:CommentLine = 'commentline'

let s:path = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:path . '/target/debug/neovim-ferris-comments'

" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "Ferris: cannot start rpc process"
  elseif -1 == id
    echoerr "Ferris: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:ferrisJobId = id 
    
    call s:configureCommands()
  endif
endfunction

function! s:configureCommands()
  command! -nargs=+ CommentLine :call s:commentline(<f-args>)
endfunction

function! s:commentline(...)
  let s:p = get(a:, 1, 0)
  let s:q = get(a:, 2, 0)

  call rpcnotify(s:ferrisJobId, s:CommentLine, str2nr(s:p), str2nr(s:q))
endfunction

" Initialize RPC
function! s:initRpc()
  if s:ferrisJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:ferrisJobId
  endif
endfunction

call s:connect()