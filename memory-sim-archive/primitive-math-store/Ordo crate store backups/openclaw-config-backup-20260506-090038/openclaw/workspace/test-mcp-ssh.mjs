import { spawn } from "node:child_process";

// Check PATH and SSH location
console.log("PATH length:", process.env.PATH?.length);
console.log("SSH location check:");
console.log("cwd:", process.cwd());
console.log("env keys:", Object.keys(process.env).length);

const which = spawn("where", ["ssh"], { stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
let whichOut = "";
let whichErr = "";
which.stdout.on("data", (d) => (whichOut += d));
which.stderr.on("data", (d) => (whichErr += d));
which.on("close", (code) => {
  console.log("where ssh exit:", code, "stdout:", whichOut.trim(), "stderr:", whichErr.trim());
});

const config = {
  target: "root@wordpress-multisites1-u70513.vm.elestio.app",
  port: 22,
  keyPath: "C:/Users/jgali/.ssh/lucernamedia_cpanel",
  strictHostKeyChecking: "accept-new",
  siteUrl: "https://wordpress-multisites1-u70513.vm.elestio.app",
  wpPath: "/var/www/html",
  dockerComposeDir: "/opt/app",
  dockerService: "wordpress",
  allowRoot: true,
  batchMode: true,
  connectTimeoutSeconds: 30,
};

const remoteCommand = "cd '/opt/app' && docker compose exec -T wordpress sh -c 'wp --path=/var/www/html --url=https://wordpress-multisites1-u70513.vm.elestio.app --allow-root option get blogname 2>/dev/null' 2>/dev/null";

const args = [
  "-T",
  "-o",
  "BatchMode=yes",
  "-o",
  `ConnectTimeout=${config.connectTimeoutSeconds}`,
  "-o",
  `StrictHostKeyChecking=${config.strictHostKeyChecking}`,
  "-i",
  config.keyPath,
  config.target,
  remoteCommand,
];

console.log("Running: ssh", args.join(" "));

const child = spawn("ssh", args, {
  stdio: ["ignore", "pipe", "pipe"],
  windowsHide: true,
});

let stdout = "";
let stderr = "";

child.stdout.on("data", (d) => (stdout += d));
child.stderr.on("data", (d) => (stderr += d));

child.on("close", (code) => {
  console.log("exit:", code);
  console.log("stdout:", stdout);
  console.log("stderr:", stderr);
});