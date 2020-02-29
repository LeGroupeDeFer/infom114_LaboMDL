import React from 'react';
import Card from 'react-bootstrap/Card';
import { loremIpsum } from '../utils/dev';
import { preview } from '../utils';
import Badge from 'react-bootstrap/Badge'
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faArrowUp, faArrowDown } from '@fortawesome/free-solid-svg-icons';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Button from 'react-bootstrap/Button';
import Moment from 'react-moment';

const Post = ({ title, text, username, vote, type, previewLength, createdOn, ...otherProps }) => {

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
    <div>

      <Card {...otherProps} className='post'>
        <Card.Header>
          <div style={{ fontSize: '19px' }}>
            <Badge className={'post-' + type}>{getDisplayedType(type)} </Badge>
            <a href='#'> {username}</a>
            <span className="text-muted" style={{ fontSize: '14px' }}> - <Moment fromNow>{createdOn}</Moment></span>
          </div>
        </Card.Header>
        <Card.Body >
          <Row>
            <Col xs='auto'>
              <Button variant="light" className='thumbsUp'> <Icon icon={faArrowUp} /></Button>
              <div className='text-center'> {vote}</div>
              <Button variant="light" className='thumbsDown'> <Icon icon={faArrowDown}></Icon></Button>

            </Col>

            <Col>
              <Card.Title>{title}</Card.Title>
              <Card.Text>{preview(text, previewLength)}</Card.Text>
            </Col>

          </Row>

        </Card.Body>
      </Card>

      <br />
    </div>


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