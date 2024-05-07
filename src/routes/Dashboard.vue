<script setup lang="ts">
import {
	Chart as ChartJS,
	Title,
	Tooltip,
	Legend,
	BarElement, ArcElement,
	CategoryScale,
	RadialLinearScale,
	PointElement,
	LineElement,
	LinearScale
} from 'chart.js'
import { Bar, Doughnut, Pie, Line, PolarArea } from 'vue-chartjs'

ChartJS.register(CategoryScale, LinearScale,
	PointElement,
	RadialLinearScale,
	LineElement, ArcElement, BarElement, Title, Tooltip, Legend)

const barData = {
	labels: [
		'January',
		'February',
		'March',
		'April',
		'May',
		'June',
		'July',
		'August',
		'September',
		'October',
		'November',
		'December'
	],
	datasets: [
		{
			label: 'Data One',
			backgroundColor: '#f87979',
			data: [40, 20, 12, 39, 10, 40, 39, 80, 40, 20, 12, 11]
		}
	]
};

const donutData = {
	labels: ['VueJs', 'EmberJs', 'ReactJs', 'AngularJs'],
	datasets: [
		{
			backgroundColor: ['#41B883', '#E46651', '#00D8FF', '#DD1B16'],
			data: [40, 20, 80, 10]
		}
	]
}

const pieData = {
	labels: ['VueJs', 'EmberJs', 'ReactJs', 'AngularJs'],
	datasets: [
		{
			backgroundColor: ['#41B883', '#E46651', '#00D8FF', '#DD1B16'],
			data: [40, 20, 80, 10]
		}
	]
};

const lineData = {
	labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
	datasets: [
		{
			label: 'Data One',
			backgroundColor: '#f87979',
			data: [40, 39, 10, 40, 39, 80, 40]
		}
	]
};

const polarData = {
	labels: [
		'Eating',
		'Drinking',
		'Sleeping',
		'Designing',
		'Coding',
		'Cycling',
		'Running'
	],
	datasets: [
		{
			label: 'My First dataset',
			backgroundColor: 'rgba(179,181,198,0.2)',
			pointBackgroundColor: 'rgba(179,181,198,1)',
			pointBorderColor: '#fff',
			pointHoverBackgroundColor: '#fff',
			pointHoverBorderColor: 'rgba(179,181,198,1)',
			data: [65, 59, 90, 81, 56, 55, 40]
		},
		{
			label: 'My Second dataset',
			backgroundColor: 'rgba(255,99,132,0.2)',
			pointBackgroundColor: 'rgba(255,99,132,1)',
			pointBorderColor: '#fff',
			pointHoverBackgroundColor: '#fff',
			pointHoverBorderColor: 'rgba(255,99,132,1)',
			data: [28, 48, 40, 19, 96, 27, 100]
		}
	]
};

const options = {
	responsive: true,
	maintainAspectRatio: false
};

</script>

<template>
	<ProjectsLayout>
		<h1 class="mt-0">Dashboard</h1>

		<p>Display an overview of all packages...</p>

		<div class="grid">
			<div class="col-4">
				<Bar :data="barData"  />
			</div>

			<div class="col-4">
				<Doughnut :data="donutData"  />
			</div>

			<div class="col-4">
				<Pie :data="pieData"  />
			</div>
		</div>

		<div class="grid">
			<div class="col-6">
				<Line :data="lineData" />
			</div>

			<div class="col-6">
				<PolarArea :data="polarData" />
			</div>
		</div>
	</ProjectsLayout>
</template>
