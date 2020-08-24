import React, { useState } from 'react';
import { Dialog, DialogTitle, DialogContent, DialogContentText, DialogActions, TextField, Button } from '@material-ui/core';

interface Props {
  open: boolean;
  onSuccess: (idx: number) => void;
  onCancel: () => void;
}

export function MonitorDialog(props: Props) {
  const [formValue, setFormValue] = useState(undefined as (string | undefined))
  return (
    <Dialog open={props.open} onClose={props.onCancel} aria-labelledby="form-dialog-title">
      <DialogTitle id="form-dialog-title">Select monitor</DialogTitle>
      <DialogContent>
        <DialogContentText>
          Select the monitor on which you wish to lock the profile:
          </DialogContentText>
        <TextField
          autoFocus
          margin="dense"
          label="Monitor ID"
          type="number"
          fullWidth
          value={formValue}
          onChange={(event: React.ChangeEvent<HTMLInputElement>) => setFormValue(event.target.value)}
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={props.onCancel} color="default">
          Cancel
          </Button>
        <Button color="primary" onClick={() => {
          const idx = Number(formValue);
          props.onSuccess(idx);
        }} disabled={Number(formValue) === NaN}>
          Lock
          </Button>
      </DialogActions>
    </Dialog>
  );
}