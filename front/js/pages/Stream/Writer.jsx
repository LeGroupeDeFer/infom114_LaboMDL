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
} from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import {
  faBalanceScale,
  faInfo,
  faLightbulb,
  faPenFancy,
  faPlusSquare, faTag,
} from '@fortawesome/free-solid-svg-icons';
import { TiDelete } from 'react-icons/ti';
import { useHistory } from 'react-router-dom';
import AutoForm from '../../components/AutoForm';
import { Option } from '../../components/SearchBar';
import {useStream} from "../../context/streamContext";
import {trace} from "../../lib";

const types = [
  { value: 'idea', label: Option({ icon: faLightbulb, label: 'Idée' }) },
  { value: 'info', label: Option({ icon: faInfo, label: 'Information' }) },
  { value: 'poll', label: Option({ icon: faBalanceScale, label: 'Sondage' }) },
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
                  <TiDelete size={20} />
                </Button>
              </InputGroup.Append>
            )}
          </InputGroup>
        </div>
      ))}

      {options.length < 5 && (
        <a href="#" onClick={addOption}>
          <Icon icon={faPlusSquare} className="mr-1" />
          <span>Ajouter une option</span>
        </a>
      )}
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

function Writer() {
  const history = useHistory();
  const stream = useStream();
  const [loading, setLoading] = useState(false);

  function onSubmit(post) {
    setLoading(true);
    if (post.kind !== 'poll') post.options = [];
    stream.posts.add(post).then(p => history.push(`/detail/${p.id}`));
  }

  const tags = stream.tags.available.map(
    t => ({ value: t.label, label: Option({ label: t.label, icon: faTag }) })
  );

  const kinds = stream.kind.available.map(
    kind => ({ value: kind.value, label: Option(kind) })
  );

  return (
    <Container className="py-5">
      <Row>
        <Col>
          <h1 className="mb-4 text-dark writer-header">
            <Icon icon={faPenFancy} className="mr-3" />
            <span>Créer un post</span>
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
}

Writer.defaultProps = {};

export default Writer;
