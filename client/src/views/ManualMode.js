import Grid from "@material-ui/core/Grid";
import Paper from "@material-ui/core/Paper";
import ManualLightSetter from "../ManualLightSetter";
import useStyles from "../style";
import React, {useEffect} from "react";
import {useMutation} from "@apollo/react-hooks";
import {gql} from "apollo-boost";

export default function ManualMode() {
  const classes = useStyles();

  const [setManual] = useMutation(gql`mutation SetManual { manualMode }`);
  useEffect(() => { setManual(); }, [] );

  return (
    <Grid container spacing={3}>
      <Grid item xs={12} md={4} lg={3}>
        <Paper className={classes.paper}>
          <ManualLightSetter lightId="TOP"/>
        </Paper>
      </Grid>
      <Grid item xs={12} md={4} lg={3}>
        <Paper className={classes.paper}>
          <ManualLightSetter lightId="BOTTOM"/>
        </Paper>
      </Grid>
      <Grid item xs={12} md={4} lg={3}>
        <Paper className={classes.paper}>
          <ManualLightSetter lightId="LEFT"/>
        </Paper>
      </Grid>
      <Grid item xs={12} md={4} lg={3}>
        <Paper className={classes.paper}>
          <ManualLightSetter lightId="RIGHT"/>
        </Paper>
      </Grid>
    </Grid>
  )
}
