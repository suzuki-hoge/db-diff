import type { Meta, StoryObj } from '@storybook/react'

import { RadioText } from './RadioText'

const meta = {
  title: 'Atoms/RadioText',
  component: RadioText,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof RadioText>

export default meta
type Story = StoryObj<typeof meta>

export const Active: Story = {
  args: {
    name: 'rdbms',
    currentValue: 'MySQL',
    value: 'MySQL',
    displayValue: 'MySQL',
  },
}

export const Inactive: Story = {
  args: {
    name: 'rdbms',
    currentValue: 'PostgreSQL',
    value: 'MySQL',
    displayValue: 'MySQL',
  },
}

export const Disable: Story = {
  args: {
    name: 'rdbms',
    currentValue: 'MySQL',
    value: 'MySQL',
    displayValue: 'MySQL',
    disabled: true,
  },
}
