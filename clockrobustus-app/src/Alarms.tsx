import { AccessAlarm, Done } from "@mui/icons-material";
import { Box, Dialog, DialogActions, DialogContent, DialogTitle, Divider, IconButton } from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { useState } from "react";
import { Alarm, dummyAlarm } from "./interfaces";
import "./Alarms.css";
import AlarmForm from "./AlarmForm";

export default function Alarms() {
  const emptyAlarms: Array<Alarm> = [];
  const [open, setOpen] = useState(false);
  const [alarms, setAlarms] = useState(emptyAlarms);
  let listener;

  const openDialog = async () => {
    try {
      const retrievedAlarms: [Alarm] = await invoke('get_alarms');
      setAlarms(retrievedAlarms);
      listener = listen('REFRESH_ALARMS', async (_e) => {
        const refreshedAlarms: [Alarm] = await invoke('get_alarms');
        
        setAlarms(refreshedAlarms);
      });
      setOpen(true);
    } catch (error) {
      alert(error);
    }
  };


  
  return (
    <div className="alarms-container">
      <Dialog open={open} scroll="paper">
        <DialogTitle>Manage Alarms</DialogTitle>
        <DialogContent>
        <Box>
          <br />
          <AlarmForm alarm={dummyAlarm} />
          <br />
          <br />
          {
            alarms.map((alarm) => {
              return(
                <AlarmForm alarm={alarm} />
              );
            })
          }
        </Box>
        </DialogContent>
        <DialogActions>
          <IconButton aria-label="done" color="secondary" onClick={() => setOpen(false)}>
            <Done />
          </IconButton>
        </DialogActions>
      </Dialog>
      <IconButton aria-label="manage alarms" color="secondary" onClick={openDialog}>
        <AccessAlarm />
      </IconButton>
    </div>
  );
}
