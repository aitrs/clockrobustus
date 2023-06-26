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
  hour: 0,
  minute: 0,
  seconds: 0,
}
