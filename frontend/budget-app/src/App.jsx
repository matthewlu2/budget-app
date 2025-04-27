import { useState, useEffect } from 'react'
import Form from './Form';
import Categories from './Categories';
import Purchases from './Purchases';
import { getter } from './Utils';

const App = () => {
	const [loading, setLoading] = useState(true);
	const [err, setErr] = useState(null);

	const [categories, setCategories] = useState([]);
	useEffect(() => {getCategories();}, []);
	const getCategories = () => {
		getter("categories", setCategories, setLoading, setErr);
	}

	const [purchases, setPurchases] = useState([]);
	useEffect(() => {getPurchases();}, []);
	const getPurchases = () => {
		getter("purchases", setPurchases, setLoading, setErr);
	}

	return (
  	<div className="App">
  		{ <Form categories = {categories} getCategories={getCategories} getPurchases={getPurchases}/> }
		{ <Categories categories = {categories}/> }
		{ <Purchases purchases = {purchases}/> }
  	</div>
	);
};

export default App;
