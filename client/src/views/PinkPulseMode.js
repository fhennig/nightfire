import useStyles from "../style";
import Grid from "@material-ui/core/Grid";
import Paper from "@material-ui/core/Paper";
import React, {useEffect} from "react";
import Title from "../Title";
import {useMutation} from "@apollo/react-hooks";
import {gql} from "apollo-boost";

export default function PinkPulseMode() {
  const classes = useStyles();

  const [setPinkPulse] = useMutation(gql`mutation SetPinkPulse { pinkPulse }`);
  useEffect(() => { setPinkPulse(); }, [] );

  return (
    <Grid container spacing={3}>
      <Grid item xs={12} md={4} lg={3}>
        <Paper className={classes.paper}>
          <Title>Pink Pulse Mode</Title>
        </Paper>
      </Grid>
    </Grid>
  )
}
