import { useState } from 'react';
import { poster } from './Utils';

const Form = ({ categories, getCategories, getPurchases}) => {
	const [categoryForm, setCategoryForm] = useState("");
	const [budgetForm, setBudgetForm] = useState("");

	const [descriptionForm, setDescriptionForm] = useState("");
	const [amountForm, setAmountForm] = useState("");
	const [dateForm, setDateForm] = useState("");
	const [catForm, setCatForm] = useState("");

	const [toggleCategory, setToggleCategory] = useState(false);
	const [togglePurchase, setTogglePurchase] = useState(false);

	const handleCategorySubmit = (e) => {
		e.preventDefault();
		const formData = new FormData(e.target);
		const formJson = Object.fromEntries(formData.entries());
		// console.log("Category form JSON:", formJson);
		poster("categories", formJson, getCategories);
		setCategoryForm("");
		setBudgetForm("");
	};	

	const handlePurchaseSubmit = (e) => {
		e.preventDefault();
		const formData = new FormData(e.target);
		const formJson = Object.fromEntries(formData.entries());
		poster("purchases", formJson, getPurchases);
		// console.log("Purchase form JSON:", formJson);
		setDescriptionForm("");
		setAmountForm("");
		setDateForm("");
		setCatForm("");
	};	

	function handleToggleCategory() {
		if (togglePurchase == false) {
			setToggleCategory(!toggleCategory);
		}
	}

	function handleTogglePurchase() {
		if (toggleCategory == false) {
			setTogglePurchase(!togglePurchase);
		}
	}

	return (
		<div>
			
		  <button onClick={() => handleToggleCategory()}>Add new category</button>
		  <button onClick={() => handleTogglePurchase()}>Add new purchase</button>

		  {toggleCategory && (
		    <form onSubmit={handleCategorySubmit}>
				<label>
					Category: <input type="text" name="name" onChange={(e) => setCategoryForm(e.target.value)} />
				</label>
				<br />
				<label>
					Monthly Budget: <input type="number" name="budget" onChange={(e) => setBudgetForm(e.target.value)} />
				</label>
				<br/>
				<input type="submit" value="Submit" />
		    </form>
		  )}

		  {togglePurchase && (
		    <form onSubmit={handlePurchaseSubmit}>
                <label>
                    Description: <input type="text" name="desc"  onChange={(e) => setDescriptionForm(e.target.value)} />
                </label>
				<br/>
                <label>
                    Amount: <input type="number" name="amount"  onChange={(e) => setAmountForm(e.target.value)} />
                </label>
                <br />
				<label>
				  Date: <input type = "date" name="date" onInput={(e) => setDateForm(e.target.value)}/>
                </label>
				<br />
				Category
				<table>
				  <tbody>
				    {categories.map((cat) => (
				      <Category key={cat.id} task={cat.name} setForm = {setCatForm}  />
				    ))}
				  </tbody>
				</table>
				<br/>
                <input type="submit" value="Submit" />
            </form>
		  )}

		</div>
	);
};

const Category = ({ task , setForm}) => {
	return (
		<tr>
			<td><input type="radio" name = "category" value = {task} onChange = {(e) => setForm(e.target.value)}/></td>
			<td>{task}</td>
		</tr>
	)
};


export default Form;
