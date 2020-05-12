import React, { useEffect, useState } from 'react';
import {
  Form,
  Button,
  Card,
  Container,
  Row,
  Col,
  InputGroup,
  Spinner,
  Tooltip,
  OverlayTrigger,
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { useHistory } from 'react-router-dom';
import { AutoForm, Authenticated } from 'unanimity/components';
import { Option } from 'unanimity/components/SearchBar';
import { useStream } from 'unanimity/context';

const types = [
  { value: 'idea', label: Option({ icon: 'lightbulb', label: 'Idée' }) },
  { value: 'info', label: Option({ icon: 'info', label: 'Information' }) },
  { value: 'poll', label: Option({ icon: 'balance-scale', label: 'Sondage' }) },
];

// I didn't find another way to add styles to the select
const primary = '#A0C55F';
const customStyles = {
  control: (base, state) => ({
    ...base,
    boxShadow: state.isFocused ? '0 0 0 1px ' + primary : 0,
    borderColor: state.isFocused ? primary : base.borderColor,
    '&:hover': {
      borderColor: state.isFocused ? primary : primary,
    },
  }),
  option: (styles, { isFocused, isSelected }) => ({
    ...styles,
    color: isSelected ? '' : '',
    backgroundColor: isFocused ? primary : null,
  }),
};

const optionsValidator = (options) =>
  options.reduce((a, option) => a && option.length, true);

function PollOptions() {
  // Otherwise, initialize the options field if not already initialized
  useEffect(() => register('options', ['', ''], true), []); // Must be before useForm!
  const { data, register, onChange } = AutoForm.useForm();
  const options = (data.options && data.options.value) || [];

  // For every kind change, check whether the kind is :
  // + not a poll => allow validity,
  // + a poll, check the values and decide on validity
  useEffect(() => {
    if (data.kind && data.kind.value !== 'poll')
      onChange('options', options, true);
    else if (
      data.kind &&
      data.kind.value === 'poll' &&
      !optionsValidator(options)
    )
      onChange('options', options, false);
  }, [data.kind]);

  // If the post is not a poll, this component ought not to be
  if (!data.kind || data.kind.value !== 'poll') {
    return <></>;
  }

  const addOption = () => onChange('options', [...options, ''], false);
  const popOption = (i) =>
    onChange(
      'options',
      options.filter((_, j) => i !== j),
      optionsValidator(options)
    );
  const updateOption = (i, value) => {
    const newOptions = options.map((option, j) => (i === j ? value : option));
    onChange('options', newOptions, optionsValidator(newOptions));
  };

  return (
    <Form.Group>
      {options.map((value, i) => (
        <div key={i}>
          <InputGroup className="pb-3">
            <Form.Control
              type="text"
              placeholder={`Option ${i + 1}`}
              value={value}
              onChange={(e) => updateOption(i, e.target.value)}
              isValid={value !== ''}
            />
            {i > 1 && (
              <InputGroup.Append>
                <Button variant="outline-danger" onClick={(_) => popOption(i)}>
                  <Icon icon="times-circle" size={20} />
                </Button>
              </InputGroup.Append>
            )}
          </InputGroup>
        </div>
      ))}

      {options.length < 5 && (
        <a href="#" onClick={addOption}>
          <Icon icon="plus-square" className="mr-1" />
          <span>Ajouter une option</span>
        </a>
      )}

      {/* <Form.Check
        type="checkbox"
        id={'checkbox-trace'}
        label={
          <>
            <span>J'aimerais avoir un suivi pour mon sondage</span>
            <OverlayTrigger
              key={name}
              placement="auto"
              overlay={
                <Tooltip className="nav-tooltip">
                  En cochant cette cache, votre sondage sera analysé et traité
                  comme une idée
                </Tooltip>
              }
            >
              <Icon icon="question-circle" className="ml-1" />
            </OverlayTrigger>
          </>
        }
        className="mt-1"
      /> */}
    </Form.Group>
  );
}

function Submit({ loading }) {
  const InnerSubmit = loading ? (
    <Spinner
      as="span"
      animation="border"
      size="sm"
      role="status"
      aria-hidden="true"
    />
  ) : (
    <span>Créer</span>
  );

  return (
    <AutoForm.Submit
      variant="primary"
      className="mt-1 float-right"
      disabled={!loading}
    >
      {InnerSubmit}
    </AutoForm.Submit>
  );
}

const Writer = Authenticated(() => {
  const history = useHistory();
  const stream = useStream();
  const [loading, setLoading] = useState(false);

  function onSubmit(post) {
    setLoading(true);
    if (post.kind !== 'poll') post.options = [];
    stream.posts.add(post).then((p) => history.push(`/detail/${p.id}`));
  }

  const tags = stream.tags.available.map((t) => ({
    value: t.label,
    label: Option({ label: t.label, icon: 'tag' }),
  }));

  const kinds = stream.kind.available.slice(1).map((kind) => ({
    value: kind.value,
    label: Option(kind),
  }));

  return (
    <Container className="py-5">
      <Row>
        <Col>
          <h1 className="mb-4 text-dark writer-header">
            <Icon icon="pen-fancy" className="mr-3" />
            <span>Écrire une publication</span>
            <hr />
          </h1>
        </Col>
      </Row>

      <Row>
        <Col>
          <Card>
            <Card.Body>
              <AutoForm onSubmit={onSubmit}>
                <Row>
                  <Col sm={12} md={6} className="pb-3">
                    <AutoForm.Select
                      id="kind"
                      name="kind"
                      options={kinds}
                      styles={customStyles}
                      placeholder="Sélectionner une catégorie"
                    />
                  </Col>
                  <Col sm={12} md={6} className="pb-3">
                    <AutoForm.Control
                      variant="primary"
                      id="title"
                      name="title"
                      type="text"
                      placeholder="Titre du post"
                    />
                  </Col>
                </Row>

                <Row className="pb-3">
                  <Col>
                    <AutoForm.Select
                      id="tags"
                      name="tags"
                      options={tags}
                      isMulti
                      placeholder={'Sélectionner un ou plusieurs tags'}
                      styles={customStyles}
                    />
                  </Col>
                </Row>

                <Row className="pb-3">
                  <Col>
                    <Form.Group>
                      <AutoForm.Control
                        optional
                        id="content"
                        name="content"
                        as="textarea"
                        rows="5"
                        placeholder="Texte..."
                      />
                    </Form.Group>
                  </Col>
                </Row>

                <PollOptions />

                <Row className="pb-3">
                  <Col>
                    <Submit loading={loading} />
                  </Col>
                </Row>
              </AutoForm>
            </Card.Body>
          </Card>
        </Col>
      </Row>
    </Container>
  );
});

Writer.defaultProps = {};

export default Writer;
