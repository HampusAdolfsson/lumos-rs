import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Button, Card, Typography, Layout } from 'antd';

export function AboutScene() {
  const [logMsgs, setLogMsgs] = useState("");

  return (
    <div style={{ textAlign: "center" }}>
      <p>Created by <strong>Hampus Adolfsson</strong>.</p>
      <p>Check out this project on <a target="_blank" href="https://github.com/HampusAdolfsson/lumos-rs">GitHub</a>!</p>
      <Card style={{ backgroundColor: "#ffffff11" }}>
        <Typography  style={{ whiteSpace: "pre-line", textAlign: "start"}}>
          {logMsgs}
        </Typography>
        <Button style={{ marginTop: 10 }} onClick={async() => {
            const logs = await invoke("get_backend_logs");
            if (typeof logs === "string") {
              setLogMsgs(logs);
            }
          }}
          type="primary"
        >Refresh Logs</Button>
      </Card>
    </div>
  );
}