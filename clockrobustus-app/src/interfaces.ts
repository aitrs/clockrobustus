import { SelectChangeEvent } from "@mui/material";
import React, { ChangeEvent } from "react";

export interface ClockMessage {
  hours: number,
  minutes: number,
  seconds: number,
  hoursAngle: number,
  minutesAngle: number,
  secondsAngle: number,
}

export interface Alarm {
  activeDays: Array<string>,
  hour: number,
  minute: number,
  seconds: number,
  id?: number,
}

export const dummyClockMessage: ClockMessage = {
  hours: 0,
  minutes: 0,
  seconds: 0,
  hoursAngle: 0,
  minutesAngle: 0,
  secondsAngle: 0,
};

export const dummyAlarm: Alarm = {
  activeDays: [],
  hour: 12,
  minute: 0,
  seconds: 0,
}

export interface AlarmFormProps {
  alarm: Alarm,
}

export interface NumericSelectProps {
  size: number,
  label: string,
  id: string,
  val: number,
  change: (event: React.ChangeEvent<HTMLInputElement>) => void,
}

export interface DayChooserProps {
  days: Array<string>,
  change: (checkedDays: Array<string>) => Promise<void>,
}
