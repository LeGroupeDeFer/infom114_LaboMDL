import React from 'react';
import { GoArrowDown } from 'react-icons/go';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Button from 'react-bootstrap/Button';
import clsx from 'clsx';

const DownVote = ({ is_logged, voted, set_vote, points, set_points }) => {
  let notLoggedMsg = 'Il faut être authentifié pour pouvoir voter';

  function downVote(e, cancel) {
    e.stopPropagation();
    if (cancel) {
      set_points(points + 1);
      set_vote('no');
    } else {
      // Case : We directly go from down to up
      if (voted == 'up') {
        set_points(points - 2);
      } else {
        set_points(points - 1);
      }
      set_vote('down');
    }
  }

  return (
    <>
      {is_logged ? (
        <Button
          variant="light"
          className={`down-vote-btn ${clsx(voted === 'down' && 'down-voted')}`}
          onClick={(e) => downVote(e, voted === 'down')}
        >
          <GoArrowDown size="1.5em" />
        </Button>
      ) : (
        <OverlayTrigger
          placement="right"
          overlay={<Tooltip> {notLoggedMsg} </Tooltip>}
        >
          <span className="d-inline-block">
            <Button variant="light" className={'down-vote-btn'} disabled>
              <GoArrowDown size="1.5em" />
            </Button>
          </span>
        </OverlayTrigger>
      )}
    </>
  );
};

DownVote.defaultProps = {
  is_logged: false,
  click_handle: null,
  voted: 'no',
};

export default DownVote;
