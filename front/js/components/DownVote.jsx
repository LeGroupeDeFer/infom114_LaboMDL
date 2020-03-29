import React from 'react';
import { GoArrowDown } from 'react-icons/go';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Button from 'react-bootstrap/Button';
import clsx from 'clsx';

const DownVote = ({ is_logged, click_handle, voted }) => {
  let notLoggedMsg = 'Il faut être authentifié pour pouvoir voter';

  return (
    <>
      {is_logged ? (
        <Button
          variant="light"
          className={`down-vote-btn ${clsx(voted === 'down' && 'down-voted')}`}
          onClick={click_handle}
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
  voted: 'no'
};

export default DownVote;
