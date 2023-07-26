import type { Meta, StoryObj } from '@storybook/react'

import { LabeledColorTagInput } from './LabeledColorTagInput'

const meta = {
  title: 'Molecules/LabeledColorTagInput',
  component: LabeledColorTagInput,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof LabeledColorTagInput>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    label: 'name',
    value: 'lorem ipsum',
    length: 20,
    onChange: console.log,
    color: 'green',
    setColor: console.log,
  },
}
