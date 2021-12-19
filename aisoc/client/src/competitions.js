import { lazy } from 'react';


export const COMPETITIONS = {
  climatehack: {
    name: 'Climate Hack',
    competition: lazy(() => import('competitions/climatehack/ClimateHack'))
  },
  uttt: {
    name: 'Ultimate Tic-Tac-Toe',
    competition: lazy(() => import('competitions/uttt/Uttt'))
  }
};

export const DEFAULT_COMPETITION = process.env.REACT_APP_DEFAULT_COMPETITION ?? 'uttt';
