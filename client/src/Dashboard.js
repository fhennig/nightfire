import React from 'react';
import clsx from 'clsx';
import CssBaseline from '@material-ui/core/CssBaseline';
import Drawer from '@material-ui/core/Drawer';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import List from '@material-ui/core/List';
import Typography from '@material-ui/core/Typography';
import Divider from '@material-ui/core/Divider';
import IconButton from '@material-ui/core/IconButton';
import Container from '@material-ui/core/Container';
import MenuIcon from '@material-ui/icons/Menu';
import ChevronLeftIcon from '@material-ui/icons/ChevronLeft';
import useStyles from "./style";
import ListSubheader from "@material-ui/core/ListSubheader";
import ListItem from "@material-ui/core/ListItem";
import ListItemIcon from "@material-ui/core/ListItemIcon";
import ListItemText from "@material-ui/core/ListItemText";
import {Link, Switch, Route} from "react-router-dom";
import PowerSettingsNewIcon from "@material-ui/icons/PowerSettingsNew";
import OffMode from "./views/OffMode";
import BrushIcon from "@material-ui/icons/Brush";
import ManualMode from "./views/ManualMode";
import FavoriteIcon from "@material-ui/icons/Favorite";
import PinkPulseMode from "./views/PinkPulseMode";



export default function Dashboard() {

  const offModePath = "/mode/off";
  const manualModePath = "/mode/manual";
  const pinkPulseModePath = "/mode/pinkpulse";

  const classes = useStyles();
  const [open, setOpen] = React.useState(true);
  const handleDrawerOpen = () => {
    setOpen(true);
  };
  const handleDrawerClose = () => {
    setOpen(false);
  };

  // problem -> calling the mutation causes a re-render and it turns into a loop.
  // new idea, use lifecycle hooks on the mode components (componentWillMount on OffMode).

  return (
    <div className={classes.root}>
      <CssBaseline/>
      <AppBar position="absolute" className={clsx(classes.appBar, open && classes.appBarShift)}>
        <Toolbar className={classes.toolbar}>
          <IconButton
            edge="start"
            color="inherit"
            aria-label="open drawer"
            onClick={handleDrawerOpen}
            className={clsx(classes.menuButton, open && classes.menuButtonHidden)}
          >
            <MenuIcon/>
          </IconButton>
          <Typography component="h1" variant="h6" color="inherit" noWrap className={classes.title}>
            Lumi
          </Typography>
        </Toolbar>
      </AppBar>
      <Drawer
        variant="permanent"
        classes={{
          paper: clsx(classes.drawerPaper, !open && classes.drawerPaperClose),
        }}
        open={open}
      >
        <div className={classes.toolbarIcon}>
          <IconButton onClick={handleDrawerClose}>
            <ChevronLeftIcon/>
          </IconButton>
        </div>
        <Divider/>
        <List>
          <ListSubheader inset>Modes</ListSubheader>
          <ListItem button component={Link} to={offModePath}>
            <ListItemIcon>
              <PowerSettingsNewIcon />
            </ListItemIcon>
            <ListItemText primary="Lights Off"/>
          </ListItem>
          <ListItem button component={Link} to={manualModePath}>
            <ListItemIcon>
              <BrushIcon />
            </ListItemIcon>
            <ListItemText primary="Manual Settings"/>
          </ListItem>
          <ListItem button component={Link} to={pinkPulseModePath}>
            <ListItemIcon>
              <FavoriteIcon />
            </ListItemIcon>
            <ListItemText primary="Pink Pulse"/>
          </ListItem>
        </List>
      </Drawer>
      <main className={classes.content}>
        <div className={classes.appBarSpacer}/>
        <Container maxWidth="lg" className={classes.container}>
          <Switch>
            <Route path={offModePath} component={OffMode}/>
            <Route path={manualModePath} component={ManualMode}/>
            <Route path={pinkPulseModePath} component={PinkPulseMode}/>
          </Switch>
        </Container>
      </main>
    </div>
  );
}
