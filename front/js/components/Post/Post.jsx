import React  from 'react';
import {Card} from 'react-bootstrap';

import { useAuth } from 'unanimity/context';
import { VOTE } from 'unanimity/lib';

import { VoteSection } from './Vote';
import { PostContent, PostFooter, PostHeader } from './index';

function Post({
  post, onVote, onFlag, onDelete, onHide, onTag, onLock,
  isPreview, onPreview, ...others
}) {

  const { user } = useAuth();
  const isLogged = !!user;
  const owner = isLogged && post.author.id === user.id;
  const voted = post.userVote !== VOTE.NONE;

  const cardProps = isPreview
    ? { onClick: e => e.target.classList.contains('expand-preview')
        ? onPreview(post)
        : null
    } : {};

  return (
    <Card {...others} {...cardProps} className={`post ${isPreview ? 'preview expand-preview' : ''}`} id={post.id}>
      <Card.Header>
        <PostHeader
          {...post}
          owner={owner}
          onHide={onHide}
          onFlag={onFlag}
          onDelete={onDelete}
          onLock={onLock}
        />
      </Card.Header>

      <Card.Body className={`p-0 ${isPreview ? 'expand-preview' : ''}`}>
        <div className={`d-flex ${isPreview ? 'expand-preview' : ''}`}>
          <VoteSection
            onVote={onVote}
            score={post.score}
            isLogged={isLogged}
            vote={post.userVote}
          />
          <div className="px-3 pb-3 pt-2">
            <Card.Text>
              <PostContent
                isPreview={isPreview}
                id={post.id}
                tags={post.tags}
                onTag={onTag}
                content={post.content}
                kind={post.kind}
                comments={post.comments}
              />
            </Card.Text>
            <PostFooter {...post} />
          </div>
        </div>
      </Card.Body>
    </Card>
  );
}


export default Post;
