import React from 'react';
import { GoArrowUp } from 'react-icons/go';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Button from 'react-bootstrap/Button';
import clsx from 'clsx';

const UpVote = ({ is_logged, voted, set_vote, points, set_points }) => {
  let notLoggedMsg = 'Il faut être authentifié pour pouvoir voter';

  function upVote(cancel) {
    if (cancel) {
      set_points(points - 1);
      set_vote('no');
    } else {
      // Case : We directly go from down to up
      if (voted == 'down') {
        set_points(points + 2);
      } else {
        set_points(points + 1);
      }
      set_vote('up');
    }
  }

  return (
    <>
      {is_logged ? (
        <Button
          variant="light"
          className={`up-vote-btn ${clsx(voted === 'up' && 'up-voted')}`}
          onClick={() => upVote(voted === 'up')}
        >
          <GoArrowUp size="1.5em" />
        </Button>
      ) : (
        <OverlayTrigger
          placement="right"
          overlay={<Tooltip> {notLoggedMsg} </Tooltip>}
        >
          <span className="d-inline-block">
            <Button variant="light" className={'up-vote-btn'} disabled>
              <GoArrowUp size="1.5em" />
            </Button>
          </span>
        </OverlayTrigger>
      )}
    </>
  );
};

UpVote.defaultProps = {
  is_logged: false,
  click_handle: null,
  voted: 'no'
};

export default UpVote;
