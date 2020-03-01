import React from 'react';
import Card from 'react-bootstrap/Card';
import { loremIpsum } from '../utils/dev';
import { preview } from '../utils';
import Badge from 'react-bootstrap/Badge'
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Button from 'react-bootstrap/Button';
import Moment from 'react-moment';
import { GoArrowDown, GoArrowUp } from "react-icons/go";
import { useState } from 'react';

const Post = ({ title, text, username, vote, type, previewLength, createdOn, currentFilter, ...otherProps }) => {

  const [voted, setVoted] = useState('no');
  const [voteH, setVoteH] = useState(vote);

  function upVote(cancel) {
    if (cancel) {
      setVoteH(voteH - 1);
      setVoted('no');
    } else {
      // Case : We directly go from down to up
      if (voted == 'down') {
        setVoteH(voteH + 2);
      } else {
        setVoteH(voteH + 1);
      }
      setVoted('up');
    }
  }

  function downVote(cancel) {
    if (cancel) {
      setVoteH(voteH + 1);
      setVoted('no');
    } else {
      // Case : We directly go from down to up
      if (voted == 'up') {
        setVoteH(voteH - 2);
      } else {
        setVoteH(voteH - 1);
      }
      setVoted('down');
    }
  }

  function getDisplayedType(type) {
    switch (type) {
      case 'info':
        return 'Information';
      case 'poll':
        return 'Poll';
      case 'idea':
        return 'Idea submission';
      case 'decisional':
        return 'Decisional';
    }
  }

  return (
    <div style={{ display: (currentFilter == 'all' || currentFilter == type) ? 'flex' : 'none' }}>

      <Card {...otherProps} className='post'>
        <Card.Header>
          <div style={{ fontSize: '19px' }}>
            <Badge className={'post-' + type}>{getDisplayedType(type)} </Badge> <a href='#'>{username}</a>
            <span className='text-muted' style={{ fontSize: '14px' }}> - <Moment fromNow>{createdOn}</Moment></span>
          </div>
        </Card.Header>
        <Card.Body style={{ padding: '1rem' }}>
          <Row>
            <Col xs='auto' className='vote-section'>
              <Button variant='light'
                className={'up-vote-btn ' + (voted == 'up' ? 'up-voted' : '')}
                onClick={() => voted != 'up' ? upVote(false) : upVote(true)}
              >
                <GoArrowUp size={26} />
              </Button>

              <div className={'text-center ' + (voted != 'no' ? voted + '-voted' : '')} style={{ fontWeight: 'bolder' }}> {voteH}</div>

              <Button variant='light'
                className={'down-vote-btn ' + (voted == 'down' ? 'down-voted' : '')}
                onClick={() => voted != 'down' ? downVote(false) : downVote(true)}
              >
                <GoArrowDown size={26} />
              </Button>

            </Col>

            <Col>
              <Card.Title>{title}</Card.Title>
              <Card.Text>{preview(text, previewLength)} <a href='#'>Read more</a></Card.Text>
            </Col>

          </Row>

        </Card.Body>
      </Card>

    </div >


  );
}

Post.defaultProps = {
  title: 'A post',
  text: loremIpsum,
  username: 'John Coffey',
  previewLength: 200,
  vote: 25,
  type: 'info',
  createdOn: '2020-02-29T12:59-0500'
};

export default Post;