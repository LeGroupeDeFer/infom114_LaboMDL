import React  from 'react';
import { Container, Row, Col } from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { components } from 'react-select';
import CreatableSelect from 'react-select/creatable';
import { useStream } from '../context/streamContext';
import { last } from '../lib';


const DropdownIndicator = (props) => {
  return (
    <components.DropdownIndicator {...props}>
      <Icon icon="search" />
    </components.DropdownIndicator>
  );
};

// Option :: Object => Component
export function Option({ icon, label }) {
  return (
    <div className="search-option">
      <span className="search-option-icon">
        <Icon icon={icon} />
      </span>
      <span className="search-option-label">{label}</span>
    </div>
  );
}

// SearchBar :: None => Component
function SearchBar({ children }) {
  const stream = useStream();

  const options = stream.tags.available.map(({ label, id }) => ({
    value: label,
    label: <Option icon="tag" label={label} />,
  }));

  const localOnChange = (options, meta) => {
    // The new option is always the last of react-select values
    const option = last(options);

    if (meta.action === 'create-option')
      stream.keywords.add(option.value);
    else if (meta.action === 'select-option')
      stream.tags.add(option.value);
    else if (meta.action === 'remove-value') {
      const v = meta.removedValue.value;
      (stream.tags.value.includes(v) ? stream.tags : stream.keywords).remove(v)
    }
  }

  return (
    <Container fluid className="search-container py-2">
      <Row>
        <Col md={4}>
          <CreatableSelect
            id="search-bar"
            className="search-bar my-1"
            classNamePrefix="search-bar"
            isMulti
            options={options}
            components={{ DropdownIndicator }}
            placeholder="Rechercher"
            formatCreateLabel={(userInput) => `Rechercher "${userInput}"`}
            onChange={localOnChange}
          />
        </Col>

        <Col md={4}>{children}</Col>
      </Row>
    </Container>
  );
}

SearchBar.Option = Option;


export default SearchBar;
