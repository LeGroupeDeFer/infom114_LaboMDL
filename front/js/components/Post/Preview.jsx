import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import Moment from 'react-moment';
import { dev, preview } from '../../lib';
import { Badge, Card, Dropdown, DropdownButton } from 'react-bootstrap';
import { DownVote, UpVote } from './Vote';
import { MdModeComment, MdReport } from 'react-icons/md';
import {
  FaTag,
  FaFacebookSquare,
  FaEllipsisH,
  FaEyeSlash,
  FaFlag,
} from 'react-icons/fa';
import clsx from 'clsx';
import { FacebookShareButton } from 'react-share';
import {useAuth} from "unanimity/context/authContext";


function getDisplayedType(type) {
  switch (type) {
    case 'info':
      return 'Information';
    case 'poll':
      return 'Vote';
    case 'idea':
      return 'Idée';
  }
}

const PostPreview = ({
 post, previewLength, currentFilter, userVote, showModal, onTagClick, ...others
}) => {

  const isLogged = !!useAuth().user;
  const {
    id, title, content, author, score, type, createdAt, comments, tags
  } = post;
  let vote = ['down', 'up', 'no'][userVote+1];

  const [voted, setVoted] = useState(vote);
  const [scoreState, setScoreState] = useState(score);

  if (!['all', type].includes(currentFilter)) return <></>;

  return (
    <div className="d-flex">
      <Card
        {...others}
        className="post"
        onClick={() => showModal(id)}
        id={id}
      >
        <Card.Header>
          <h5>
            <Badge className={`post-${type} mr-2`}>
              {getDisplayedType(type)}
            </Badge>
            <span className="mr-2">{title}</span>

            <span className="text-muted">
              {' '}
              <a href="#" className="text-dark">
                {author.firstname}
                {'  '}
                {author.lastname}
              </a>{' '}
              -{' '}
              <Moment locale="fr" fromNow>
                {createdAt}
              </Moment>
            </span>

            <DropdownButton
              title={
                <span>
                  <FaEllipsisH />
                </span>
              }
              variant="link"
              className="float-right more btn-link"
              onClick={(e) => e.stopPropagation()}
            >
              <Dropdown.Item as="button">
                <FaEyeSlash className="mr-2" />
                Masquer
              </Dropdown.Item>
              <Dropdown.Item as="button">
                <FaFlag className="mr-2" />
                Signaler
              </Dropdown.Item>
            </DropdownButton>
          </h5>
        </Card.Header>

        <Card.Body className="p-0">
          <div className="d-flex">
            <div className="vote-section">
              <UpVote
                isLogged={isLogged}
                voted={voted}
                set_vote={setVoted}
                score={scoreState}
                set_score={setScoreState}
                post_id={id}
              />
              <div
                className={`text-center ${clsx(
                  voted !== 'no' && voted + '-voted'
                )}`}
              >
                <b>{scoreState}</b>
              </div>

              <DownVote
                isLogged={isLogged}
                voted={voted}
                set_vote={setVoted}
                score={scoreState}
                set_score={setScoreState}
                post_id={id}
              />
            </div>

            <div className="px-3 pb-3 pt-2">
              <div className="mb-1">

                {tags.map((tag, index) => {
                  return (
                    <a
                      href="#"
                      className="mr-2 tag"
                      onClick={(e) => onTagClick(e)}
                      value={tag}
                      key={index}
                    >
                      <FaTag className="mr-1" />
                      {tag}
                    </a>);
                })}

              </div>

              <Card.Text>
                {preview(content, previewLength)}{' '}
                <Link to={'/post/' + id}>Lire la suite</Link>
              </Card.Text>

              <Link
                to={'/post/' + id}
                className="post-footer-btn mr-2"
                href="#"
              >
                <MdModeComment size="1.25em" className="mr-1" />
                <span className="text-muted">
                  {comments.length}{' '}
                  {comments.length <= 1 ? 'commentaire' : 'commentaires'}
                </span>
              </Link>

              <FacebookShareButton
                url={'https://unanimity.be/post/' + id}
                quote={title + ' - ' + author.firstname + ' ' + author.lastname}
                onClick={(e) => e.stopPropagation()}
              >
                <a className="post-footer-btn mr-2" href="#">
                  <FaFacebookSquare size="1.25em" className="mr-1" />
                  <span className="text-muted">Partager</span>
                </a>
              </FacebookShareButton>
            </div>
          </div>
        </Card.Body>
      </Card>
    </div>
  );

};


export default PostPreview;
