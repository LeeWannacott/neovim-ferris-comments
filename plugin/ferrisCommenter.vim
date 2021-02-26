" Initialize the channel
if ! exists('s:ferrisjobid')
	let s:ferrisjobid = 0
endif

let s:path = resolve(expand('<sfile>:p:h') . '/..')

let s:bin = s:path . '/target/debug/neovim-ferris-comments'

" Constants for RPC messages.
let s:Comment = 'comment'


" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:init()
  let id = s:GetJobId()
  
  if 0 == id
    echoerr "commenter: cannot start rpc process"
  elseif -1 == id
    echoerr "commenter: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:ferrisjobid = id 
    
    call s:configureCommands(id)
  endif
endfunction

function! s:configureCommands(jobid)
  " command! -nargs=+ Add :call s:add(<f-args>)
  command! -nargs=+ Comment :call s:comment(<f-args>)
endfunction

function! s:comment(...)
  let s:p = get(a:, 1)

  call rpcnotify(s:ferrisjobid, s:Comment, str2nr(s:p))
endfunction

function! s:OnStderr(id, data, event) dict
  echom 'ferris commenter: stderr: ' . join(a:data, "\n")
endfunction

" Initialize RPC
function! s:GetJobId()
  if s:ferrisjobid == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true, 'on_stderr': function('s:OnStderr')})
    return jobid
  else
    return s:ferrisjobid
  endif
endfunction

" Send an RPC message to the remote process.
function! s:rpc(rpcMessage)
	call rpcnotify(s:ferrisjobid, a:rpcMessage)
endfunction

call s:init()