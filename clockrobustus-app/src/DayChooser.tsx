import { Checkbox, FormControlLabel, FormGroup } from "@mui/material";
import { useEffect, useState } from "react";
import { DayChooserProps } from "./interfaces";

export default function DayChooser(props: DayChooserProps) {
  const emptyArray: Array<string> = [];
  const [daysChecked, setDaysChecked] = useState(emptyArray);
  useEffect(() => {
    setDaysChecked(props.days);
  });
  const dayLabels = [
    ["Mon", "Monday"],
    ["Tue", "Tuesday"],
    ["Wed", "Wednesday"],
    ["Thu", "Thursday"],
    ["Fri", "Friday"],
    ["Sat", "Saturday"],
    ["Sun", "Sunday"],
  ];

  return(
    <FormGroup aria-label="row checkboxes" row>
    {
      dayLabels.map((d) => {
        return (<FormControlLabel
          style={{ transform: 'scale(0.8)', width: '10px' }}
          onChange={(event: any) => {
            const index = daysChecked.findIndex((elt) => elt === d[1]);

            if (index !== -1 && !event.target.checked) {
              let newDays = [...daysChecked] as Array<string>;
              newDays.splice(index, 1);
              setDaysChecked(newDays);
              props.change(newDays);
            } else if (event.target.checked) {
              let newDays = [...daysChecked] as Array<string>;
              newDays.push(d[1]);
              setDaysChecked(newDays);
              props.change(newDays);
            }
          }}
          value={d[1]}
          control={<Checkbox />}
          label={d[0]}
          labelPlacement="top"
          checked={
            daysChecked
              .findIndex((elt) => elt === d[1]) !== -1
          }
        />);
      })
    }
    </FormGroup>
  );
}
