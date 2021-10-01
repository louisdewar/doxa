import Navbar from 'component/NavBar.js';
import './Uttt.scss';

import React from 'react';
import Leaderboard from 'component/Leaderboard';

function Uttt() {
  return (
    <div>
      <Navbar competitionName='Ultimate Tic-Tac-Toe' homepageUrl='/c/uttt/'/>
      <div className='main'>
        <div className="comp-info">
          <div className="header maxwidth">
            <h1>ULTIMATE TIC-TAC-TOE</h1>
          </div>
          <div className="about maxwidth">
            <h3>Description</h3>
            <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam in dolor maximus, efficitur nisi non, scelerisque eros. Mauris pulvinar placerat elit, in scelerisque diam luctus ac. Phasellus elit mauris, euismod id ullamcorper eget, mattis ut velit. Morbi interdum velit ut bibendum facilisis. Vivamus dolor libero, finibus at scelerisque ut, porttitor vitae tellus. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Morbi efficitur nibh eget elit egestas viverra et in elit.</p>
            <form className='comp-buttons'>
              <input type='button' value='RULES' />
              <input type='button' value='HOW TO PARTICIPATE' />
              <input type='button' value='ENROL NOW' />
            </form>
          </div>
        </div>
        <Leaderboard />
      </div>
    </div>
  );
}

export default Uttt;