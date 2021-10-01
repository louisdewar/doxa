import { BASE_URL, request } from './common';

const GAME_BASE_URL = BASE_URL + 'competition/uttt/_game/';

export async function getEvents(game, filter) {
  const data = await request({
    url: GAME_BASE_URL + game + '/events',
    method: 'GET',
    params: filter? { t: filter }: null, 
  });

  return data.events;
}

export async function getSingleEvent(game, filter) {
  const events = await getEvents(game, filter);
  const event = events[0];

  return event;
}

export async function getPlayers(game) {
  const data = await request({
    url: GAME_BASE_URL + game + '/players',
    method: 'GET', 
  });

  return data.players;
}

export async function getResult(game, agent) {
  const data = await request({
    url: GAME_BASE_URL + game + '/result/' + agent,
    method: 'GET', 
  });

  return data.result;
}

export async function getUTTTGameWinners(game) {
  const data = await getSingleEvent(game, 'game_winners');

  return data.payload.winners;
}

export async function getUTTTGameScores(game) {
  const data = await getSingleEvent(game, 'scores');

  return data.payload;
}

export async function getUTTTGameEvents(match, subGameID) {
  const data = await getSingleEvent(match, 'game_' + subGameID);

  return data.payload.events;
}

export default { getEvents, getPlayers, getUTTTGameWinners, getUTTTGameEvents, getResult, getUTTTGameScores };