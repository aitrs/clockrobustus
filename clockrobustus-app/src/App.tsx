import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import "./App.css";
import { ClockMessage, dummyAlarm, dummyClockMessage } from "./interfaces";
import Alarms from "./Alarms";
import NumericSelect from "./NumericSelect";
import AlarmForm from "./AlarmForm";

function formatDigits(d: number): string {
  return d.toLocaleString('en-US', {
    minimumIntegerDigits: 2,
    useGrouping: false,
  });
}

function App() {
  const [clock, setClock] = useState(dummyClockMessage);
  const [invoked, setInvoked] = useState(false);

  React.useEffect(() => {
    (async function() {
      await appWindow.listen(
        'CLOCK',
        (evt) => {
          setClock(evt.payload as ClockMessage);
        }
      );
      if (!invoked) {
        await invoke('clock_events', {
          window: appWindow,
        });
        setInvoked(true);
      }
    })();
  });
  return (
    <div className="container">
      <div className="face">
        <div 
          id="hourHand"
          style={{ transform: `rotate(${clock.hoursAngle}rad)`}}>
        </div>
        <div
          id="minuteHand"
          style={{ transform: `rotate(${clock.minutesAngle}rad)`}}>
        </div>
        <div
          id="secondHand"
          style={{ transform: `rotate(${clock.secondsAngle}rad)`}}>
        </div>
        <Alarms />
        <p className="digital">
          {formatDigits(clock.hours)}
          :{formatDigits(clock.minutes)}
          :{formatDigits(clock.seconds)}
        </p>
      </div>
    </div>
  );
}

export default App;
