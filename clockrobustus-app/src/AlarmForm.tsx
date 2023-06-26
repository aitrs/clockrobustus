import { Delete, Save } from "@mui/icons-material";
import { Box, Divider, IconButton } from "@mui/material";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";
import DayChooser from "./DayChooser";
import { AlarmFormProps, dummyAlarm } from "./interfaces";
import NumericSelect from "./NumericSelect";

export default function AlarmForm(props: AlarmFormProps) {
  const [alarm, setAlarm] = useState(dummyAlarm);

  useEffect(() => {
    setAlarm(props.alarm);
  });

  let deleteButton;

  if (alarm.id) {
    deleteButton = <IconButton 
      aria-label="delete alarm"
      onClick={async () => {
        await invoke('delete_alarm', {
          alarm,
        });
        appWindow.emit('REFRESH_ALARMS', {});
      }}
    >
      <Delete />
    </IconButton>;
  } else {
    deleteButton = <IconButton
      aria-label="save alarm"
      onClick={async () => {
        await invoke('upsert_alarm', {
          alarm,
        });
        appWindow.emit('REFRESH_ALARMS', {});
      }}
    >
      <Save />
    </IconButton>;
  }

  return(
    <Box>
      <Box sx={{ minWidth: 200, display: 'inline-flex' }}>
        <NumericSelect 
          id={`alarm-id-hour-${alarm.id || 0}`}
          size={24}
          label="Hour"
          val={alarm.id ? alarm.hour : 12}
          change={async (event) => {
            alarm.hour = parseInt(event.target.value);
            if (alarm.id) {
              await invoke('upsert_alarm', {
                alarm
              });
              appWindow.emit('REFRESH_ALARMS', {});
            }
          }}
        />
        <NumericSelect 
          id={`alarm-id-minute-${alarm.id || 0}`}
          size={60}
          label="Minute"
          val={alarm.id ? alarm.minute : 0}
          change={async (event) => {
            alarm.minute = parseInt(event.target.value);
            if (alarm.id) {
              await invoke('upsert_alarm', {
                alarm
              });
              appWindow.emit('REFRESH_ALARMS', {});
            }
          }}
        />
        <NumericSelect 
          id={`alarm-id-second-${alarm.id || 0}`}
          size={60}
          label="Second"
          val={alarm.id ? alarm.seconds : 0}
          change={async (event) => {
            alarm.seconds = parseInt(event.target.value);
            if (alarm.id) {
              await invoke('upsert_alarm', {
                alarm
              });
              appWindow.emit('REFRESH_ALARMS', {});
            }
          }}
        />
      </Box>
      <Box sx={{ minWidth: 200, display: 'inline-flex'}}>
        <DayChooser 
          days={alarm.activeDays}
          change={async (days) => {
            alarm.activeDays = days;

            if (alarm.id) {
              await invoke('upsert_alarm', {
                alarm,
              });
              appWindow.emit('REFRESH_ALARMS', {});
            }
          }}
        />
        {deleteButton}
      </Box>
      <br />
      <Divider />
      <br />
    </Box>
  );
}
