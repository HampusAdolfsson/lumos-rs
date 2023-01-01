import { useEffect, useState } from 'react';
import { Button, TextField } from '@mui/material';
import Box from '@mui/material/Box';
import { invoke } from '@tauri-apps/api/tauri';

export function AboutScene() {
  const [logMsgs, setLogMsgs] = useState("");

  return (
    <Box sx={{ textAlign: "center", paddingTop: "1em" }}>
      <p>Created by <strong>Hampus Adolfsson</strong>.</p>
      <p>Check out this project on <a href="https://github.com/HampusAdolfsson/lumos-rs">GitHub</a>!</p>
      <TextField sx={{ width: 600, "& .MuiInputBase-input": { fontSize: 12 } }}
        value={logMsgs}
        variant="outlined"
        rows={14}
        disabled
        multiline />
      <Box sx={{ width: "100%", marginTop: "10px" }}>
        <Button onClick={async() => {
          const logs = await invoke("get_backend_logs");
          if (typeof logs === "string") {
            setLogMsgs(logs);
          }
          }} sx={{ display: "block", margin: "auto" }}
          variant="outlined" size="small"
        >Refresh Logs</Button>
      </Box>
    </Box>
  );
}