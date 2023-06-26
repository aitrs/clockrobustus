import { FormControl, InputLabel, MenuItem, Select } from "@mui/material";
import { NumericSelectProps } from "./interfaces";

export default function NumericSelect(props: NumericSelectProps) {
  let items = Array(props.size).fill(0);

  return(
    <FormControl fullWidth>
      <InputLabel id={`${props.id}-label`}>{props.label}</InputLabel>
      <Select
        labelId={`${props.id}-label`}
        id={props.id}
        value={`${props.val}`}
        label={props.label}
        onChange={props.change}
      >
        {
          [...items.keys()].map((k) => {
            return(
              <MenuItem value={k}>{k}</MenuItem>
            );
          })
        }
      </Select> 
    </FormControl>
  );
}
