import type { Meta, StoryObj } from '@storybook/react'

import { LabeledInputText } from './LabeledInputText'

const meta = {
  title: 'Molecules/LabeledInputText',
  component: LabeledInputText,
  tags: ['autodocs'],
  argTypes: {},
} satisfies Meta<typeof LabeledInputText>

export default meta
type Story = StoryObj<typeof meta>

export const Component: Story = {
  args: {
    label: 'Schema Name',
    maxLength: 20,
    chars: 'all',
  },
}
