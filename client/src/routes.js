import BrushIcon from '@material-ui/icons/Brush';
import PowerSettingsNewIcon from '@material-ui/icons/PowerSettingsNew';
import FavoriteIcon from '@material-ui/icons/Favorite';
import ManualMode from "./views/ManualMode";
import OffMode from "./views/OffMode";
import PinkPulseMode from "./views/PinkPulseMode";


export const DashboardRoutes = [
  {
    path: "/off",
    title: "Lights Off",
    icon: PowerSettingsNewIcon,
    component: OffMode
  },
  {
    path: "/manual",
    title: "Manual Settings",
    icon: BrushIcon,
    component: ManualMode
  },
  {
    path: "/pinkpulse",
    title: "PinkPulse",
    icon: FavoriteIcon,
    component: PinkPulseMode
  }
];

export default {
  DashboardRoutes,
}
