import { lazy } from 'react';


export const COMPETITIONS = {
  climatehack: {
    name: 'Climate Hack',
    competition: lazy(() => import('competitions/climatehack/ClimateHack'))
  },
  uttt2: {
    name: 'Ultimate Tic-Tac-Toe',
    competition: lazy(() => import('competitions/uttt2/Uttt'))
  }
};

export const DEFAULT_COMPETITION = process.env.REACT_APP_DEFAULT_COMPETITION ?? 'uttt2';
