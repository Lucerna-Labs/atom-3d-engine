$blob = "C:\Users\jgali\.ollama\models\blobs\sha256-2a5266475777daf23e21991592a006f6dbc0be4620ac6a12d7d57240b0027711"
$cv   = "C:\Projects\Rust Simulation for Synthetic Memory Specialist\data\answer_mode_cv.gguf"
$proj = "C:\Projects\Rust Simulation for Synthetic Memory Specialist"
$port = 8092

function RunCond($tag, $extra) {
  $a = @("-m", $blob, "-ngl", "99", "--port", "$port", "--host", "127.0.0.1", "-c", "4096") + $extra
  $err = "$proj\rag_runs\srv_$tag.err.log"
  $out = "$proj\rag_runs\srv_$tag.out.log"
  $p = Start-Process -FilePath "llama-server" -ArgumentList $a -PassThru -WindowStyle Hidden -RedirectStandardError $err -RedirectStandardOutput $out
  $env:LS_URL = "http://127.0.0.1:$port"; $env:LS_TAG = $tag; $env:LS_SUITES = "dev,holdout"
  python "$proj\scripts\eval_llamaserver.py"
  Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
  Start-Sleep -Seconds 4
}

RunCond "bare" @()
foreach ($s in @(-6, 3, 6, 9, 12)) {
  RunCond "cv-s$s" @("--control-vector-scaled", $cv, "$s")
}
Write-Output "CV_EVAL_DONE"
