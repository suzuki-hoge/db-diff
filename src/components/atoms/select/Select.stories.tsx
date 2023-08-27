import type { Meta, StoryObj } from '@storybook/react'

import { Select } from './Select'

const meta = {
  title: 'Atoms/Select',
  component: Select,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof Select>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    defaultValue: 'two',
    values: ['one', 'two', 'three'],
    onChange: console.log,
  },
}
