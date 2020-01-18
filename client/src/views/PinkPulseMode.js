import useStyles from "../style";
import Grid from "@material-ui/core/Grid";
import Paper from "@material-ui/core/Paper";
import React from "react";
import Title from "../Title";

export default function PinkPulseMode() {
  const classes = useStyles();

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
