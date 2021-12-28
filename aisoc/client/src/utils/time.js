/**
 * Adapted from bahamas10/human (https://github.com/bahamas10/human/blob/master/human.js),
 * available as the human-time NPM package (https://www.npmjs.com/package/human-time).
 */

const names = ['year', 'month', 'week', 'day', 'hour', 'minute', 'second'];

export function formatDuration(seconds) {
  seconds = Math.abs(seconds);
  const times = [
    seconds / 60 / 60 / 24 / 365, // years
    seconds / 60 / 60 / 24 / 30,  // months
    seconds / 60 / 60 / 24 / 7,   // weeks
    seconds / 60 / 60 / 24,       // days
    seconds / 60 / 60,            // hours
    seconds / 60,                 // minutes
    seconds                       // seconds
  ];

  for (let i = 0; i < names.length; i++) {
    const time = Math.floor(times[i]);
    if (time >= 1) {
      return time + ' ' + names[i] + (time > 1 ? 's' : '');
    }
  }

  return '0 seconds';
}

export function formatTime(seconds) {
  if (seconds instanceof Date) {
    seconds = Math.round((Date.now() - seconds) / 1000);
  }

  return formatDuration(seconds) + (seconds < 0 ? ' from now' : ' ago');
}
