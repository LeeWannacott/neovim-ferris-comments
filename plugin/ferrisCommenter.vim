" Initialize the channel
if ! exists('s:commenterJobId')
	let s:commenterJobId = 0
endif

let s:path = resolve(expand('<sfile>:p:h') . '/..')

let s:bin = s:path . '/target/debug/neovim-ferris-comments'

" Constants for RPC messages.
let s:CommentLine = 'comment'


" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "commenter: cannot start rpc process"
  elseif -1 == id
    echoerr "commenter: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:commenterJobId = id 
    
    call s:configureCommands()
  endif
endfunction

function! s:configureCommands()
  " command! -nargs=+ Add :call s:add(<f-args>)
endfunction

" Initialize RPC
function! s:initRpc()
  if s:commenterJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true})
    return jobid
  else
    return s:commenterJobId
  endif
endfunction

call s:connect()